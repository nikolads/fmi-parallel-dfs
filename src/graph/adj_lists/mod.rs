use rand::prelude::*;
use rayon::{self, prelude::*};
use std::iter;

use crate::graph::{Edge, GraphRef, Prng};

mod job;
pub mod mirror;

use self::job::JobDesc;

/// Simple graph represented using adjacency lists.
///
/// Vertices are represented with integer ids in `0..n_verts`.
/// An edge *(u, v)* from `u` to `v` is represented by storing `v`
/// in the vector `lists[u]`.
///
/// For example the graph with vertices {0, 1, 2} and edges
/// {(0, 1), (0, 2), (1, 2)} is represented by
///
/// ```ignore
/// lists = [
///     [1, 2], // index 0: edges (0, 1) and (0, 2)
///     [2],    // index 1: edges (1, 2)
///     [],     // index 2: no edges
/// ]
/// ```
///
/// The graph is directed. An undirected graph is represented
/// by adding an edge in both directions (ie *(u, v)* and *(v, u)*).
/// Loops (*(u, u)* edges) and multiple edges are not allowed.
#[derive(Debug, Clone)]
pub struct AdjLists {
    n_verts: usize,
    lists: Vec<Vec<usize>>,
}

impl AdjLists {
    /// Create new empty graph
    pub fn new(n_verts: usize) -> Self {
        AdjLists {
            n_verts,
            lists: vec![vec![]; n_verts],
        }
    }

    /// Create new directed graph with randomly generated edges.
    ///
    /// Creates a graph with `n_verts` vertices and `n_edges` randomly generated
    /// edges. The job is automatically parallelized by `rayon`.
    ///
    /// `seeds` is an iterator with initial states to use for local random
    /// number generators if reproducibility is required. If there aren't
    /// enough elements in the iterator random seeds will be chosen. `None`
    /// can be passed to use entirely random seeds.
    ///
    /// # Panics
    ///
    /// If `n_edges` is more than the edges of a full graph with `n_verts`
    /// vertices, i.e. (`n_verts  * (n_verts - 1)`)
    pub fn gen_directed<I>(n_verts: usize, n_edges: usize, seeds: I) -> Self
    where
        I: IntoIterator<Item = <Prng as SeedableRng>::Seed>,
    {
        assert!(n_edges <= n_verts * (n_verts - 1));

        let mut graph = AdjLists::new(n_verts);

        // Number of vertices bellow which we prefer to calculate sequentially
        // TODO: benchmark to choose an appropriate value
        // TODO: should we parallelize over number of edges instead?
        const VERTS_PER_CHUNK: usize = 128;

        // Calculate the number of seeds that we will need and pre-collect them in a
        // vector. We need this because we can't share a mutable iterator
        // between threads without locking.
        let seeds = seeds
            .into_iter()
            .map(|s| Some(s))
            .chain(iter::repeat(None))
            .take(graph.lists.chunks(VERTS_PER_CHUNK).count())
            .collect::<Vec<_>>();

        JobDesc {
            n_verts,
            n_edges,
            lists: &mut graph.lists,
            directed: true,
        }
        .chunked(VERTS_PER_CHUNK)
        .zip(seeds)
        .for_each(|(mut job, seed)| job.gen(seed));

        graph
    }

    /// Create new directed graph with randomly generated edges.
    ///
    /// Creates a graph with `n_verts` vertices and `n_edges` randomly generated
    /// edges. The job is manually split between `n_threads` threads.
    ///
    /// This exists mostly to benchmark against `gen_directed`.
    ///
    /// # Panics
    ///
    /// If `n_edges` is more than the edges of a full graph with `n_verts`
    /// vertices, i.e. (`n_verts  * (n_verts - 1)`)
    pub fn gen_directed_on_threads<I>(
        n_verts: usize,
        n_edges: usize,
        n_threads: usize,
        seeds: I,
    ) -> Self
    where
        I: IntoIterator<Item = <Prng as SeedableRng>::Seed>,
        <I as IntoIterator>::IntoIter: Send,
    {
        assert!(n_edges <= n_verts * (n_verts - 1));

        let mut graph = AdjLists::new(n_verts);
        let seeds = seeds.into_iter().map(|s| Some(s)).chain(iter::repeat(None));

        rayon::scope(|scope| {
            JobDesc {
                n_verts,
                n_edges,
                lists: &mut graph.lists,
                directed: true,
            }
            .threaded(n_threads)
            .zip(seeds)
            .for_each(|(mut job, seed)| scope.spawn(move |_| job.gen(seed)));
        });

        graph
    }

    /// Create new undirected graph with randomly generated edges.
    ///
    /// Creates a graph with `n_verts` vertices and a total of `2 * n_edges`
    /// edges. Edges are created symmetrically, i.e. if *(u, v)* exists then
    /// *(v, u)* exists too.
    ///
    /// The job is automatically parallelized by `rayon`.
    /// `seeds` is an iterator with initial states to use for local random
    /// number generators if reproducibility is required. If there aren't
    /// enough elements in the iterator random seeds will be chosen. `None`
    /// can be passed to use entirely random seeds.
    ///
    /// # Panics
    ///
    /// If `2 * n_edges` is more than the edges of a full graph with `n_verts`
    /// vertices, i.e. (`n_verts  * (n_verts - 1)`)
    pub fn gen_undirected<I>(n_verts: usize, n_edges: usize, seeds: I) -> Self
    where
        I: IntoIterator<Item = <Prng as SeedableRng>::Seed>,
    {
        assert!(n_edges <= n_verts * (n_verts - 1) / 2);

        const VERTS_PER_CHUNK: usize = 128;

        let mut graph = AdjLists::new(n_verts);

        let seeds = seeds
            .into_iter()
            .map(|s| Some(s))
            .chain(iter::repeat(None))
            .take(graph.lists.chunks(VERTS_PER_CHUNK).count())
            .collect::<Vec<_>>();

        JobDesc {
            n_verts,
            n_edges,
            lists: &mut graph.lists,
            directed: false,
        }
        .chunked(VERTS_PER_CHUNK)
        .zip(seeds)
        .for_each(|(mut job, seed)| job.gen(seed));

        mirror::seq(&mut graph.lists);

        graph
    }

    /// Sort the graph, so that edges come in order for `edges` and
    /// `neighbours`.
    ///
    /// Uses `rayon` for pararellism.
    pub fn sort(&mut self) {
        self.lists
            .par_iter_mut()
            .for_each(|list| list.sort_unstable())
    }

    /// Iterator over all vertices in the graph.
    pub fn vertices(&self) -> std::ops::Range<usize> {
        0..self.n_verts
    }

    /// Rayon parallel iterator over all vertices in the graph.
    pub fn vertices_par(&self) -> rayon::range::Iter<usize> {
        (0..self.n_verts).into_par_iter()
    }

    /// Iterator over all edges in the graph.
    pub fn edges<'a>(&'a self) -> impl Iterator<Item = Edge> + 'a {
        self.vertices()
            .flat_map(move |v| iter::repeat(v).zip(self.neighbours(v)))
            .map(|(from, to)| Edge::new(from, to))
    }

    /// Iterator over the neighbours of vertex `v`.
    ///
    /// The neighbours are all vertices `u` such that an edge from `v` to `u`
    /// exists.
    pub fn neighbours<'a>(&'a self, v: usize) -> std::iter::Cloned<std::slice::Iter<'a, usize>> {
        self.lists[v].iter().cloned()
    }
}

impl<'a> GraphRef<'a> for &'a AdjLists {
    type Vertices = std::ops::Range<usize>;
    type VerticesPar = rayon::range::Iter<usize>;
    type Neighbours = std::iter::Cloned<std::slice::Iter<'a, usize>>;

    fn vertices(self) -> Self::Vertices {
        self.vertices()
    }

    fn vertices_par(self) -> Self::VerticesPar {
        self.vertices_par()
    }

    fn neighbours(self, v: usize) -> Self::Neighbours {
        self.neighbours(v)
    }
}

#[cfg(test)]
mod tests;
