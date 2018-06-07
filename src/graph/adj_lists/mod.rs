use rand;
use rand::distributions::Uniform;
use rand::prelude::*;
use rayon;
use rayon::prelude::*;

use graph::Edge;

use std::cmp;
use std::iter;
use std::mem;
use std::ops::Range;

/// Pseudo-random number generator algorithm used in this module.
///
/// Using XorShift because it is currently the only
/// non-cryptographically secure PRNG provided by `rand`.
pub type Prng = rand::prng::XorShiftRng;

/// Simple graph represented using adjacency lists.
///
/// Vertices are represented with integer ids in `0..n_verts`.
/// An edge *(u, v)* from `u` to `v` is represented by storing `v`
/// in the vector `lists[u]`.
///
/// For example the graph with vertices {0, 1, 2} and edges
/// {(0, 1), (0, 2), (1, 2)} is represented by
///
/// ```
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
///
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
    /// Creates a graph with `n_verts` vertices and `n_edges` randomly generated edges.
    /// The job is automatically parallelized by `rayon`.
    ///
    /// `seeds` is an iterator with initial states to use for local random number generators
    /// if reproducibility is required. If there aren't enough elements in the iterator
    /// random seeds will be chosen.
    ///
    /// # Panics
    ///
    /// If `n_edges` is more than the edges of a full graph with `n_verts` vertices (`n_verts  *
    /// (n_verts - 1)`)
    ///
    pub fn gen_directed<I>(n_verts: usize, n_edges: usize, seeds: I) -> Self
    where
        I: IntoIterator<Item = <Prng as SeedableRng>::Seed>,
    {
        assert!(n_edges <= n_verts * (n_verts - 1));

        // Number of vertices bellow which we prefer to calculate sequentially instead of
        // parallelizing across multiple tasks.
        // TODO: benchmark to choose an appropriate value
        // TODO: should we parallelize over number of edges instead?
        const VERTS_PER_CHUNK: usize = 128;

        let mut graph = AdjLists::new(n_verts);

        // Calculate the number of seeds that we will need and pre-collect them in a vector.
        // We need this because we can't share a mutable iterator between threads without locking.
        let seeds = seeds
            .into_iter()
            .map(|s| Some(s))
            .chain(iter::repeat(None))
            .take(graph.lists.chunks(VERTS_PER_CHUNK).count())
            .collect::<Vec<_>>();

        graph
            .lists
            .par_chunks_mut(VERTS_PER_CHUNK)
            .enumerate()
            .zip(seeds)
            .for_each(|((i, lists), seed)| {
                let edges = Self::subrange(
                    0..n_edges,
                    i * VERTS_PER_CHUNK,
                    i * VERTS_PER_CHUNK + lists.len(),
                    n_verts
                ).len();

                DirectedPart {
                    from_verts: (i * VERTS_PER_CHUNK)..cmp::min((i + 1) * VERTS_PER_CHUNK, n_verts),
                    to_verts: 0..n_verts,
                    lists,
                }.gen(edges, seed);
            });

        graph
    }

    /// Create new directed graph with randomly generated edges.
    ///
    /// Creates a graph with `n_verts` vertices and `n_edges` randomly generated edges.
    /// The job is manually split between `n_threads` threads.
    ///
    /// This exists mostly to benchmark against `gen_directed`.
    ///
    /// # Panics
    ///
    /// If `n_edges` is more than the edges of a full graph with `n_verts` vertices (`n_verts  *
    /// (n_verts - 1)`)
    ///
    pub fn gen_directed_on_threads<I>(n_verts: usize, n_edges: usize, n_threads: usize, seeds: I) -> Self
    where
        I: IntoIterator<Item = <Prng as SeedableRng>::Seed>,
        <I as IntoIterator>::IntoIter: Send,
    {
        assert!(n_edges <= n_verts * (n_verts - 1));

        let mut graph = AdjLists::new(n_verts);

        let mut state = &mut graph.lists[..];
        let mut seeds = seeds.into_iter();

        rayon::scope(|scope| {
            for i in 0..n_threads {
                let from_verts = Self::subrange(0..n_verts, i, i + 1, n_threads);

                // `mem::replace` to "trick" the borrow checker
                let tmp = mem::replace(&mut state, &mut []);
                let (lists, next) = tmp.split_at_mut(from_verts.end - from_verts.start);
                state = next;

                let mut part = DirectedPart {
                    from_verts: from_verts,
                    to_verts: 0..n_verts,
                    lists: lists,
                };

                let edges = Self::subrange(0..n_edges, i, i + 1, n_threads).len();
                let seed = seeds.next();

                scope.spawn(move |_| part.gen(edges, seed));
            }
        });

        graph
    }

    /// Sort the graph, so that edges come in order for `edges` and `neighbours`.
    ///
    /// Uses `rayon` for pararellism.
    pub fn sort(&mut self) {
        self.lists.par_iter_mut().for_each(|list| list.sort_unstable())
    }

    /// Returns iterator over all vertices in the graph.
    pub fn vertices<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        0..self.n_verts
    }

    pub fn vertices_par<'a>(&'a self) -> impl ParallelIterator<Item = usize> + 'a {
        (0..self.n_verts).into_par_iter()
    }

    /// Returns iterator over all edges in the graph.
    pub fn edges<'a>(&'a self) -> impl Iterator<Item = Edge> + 'a {
        self.vertices()
            .flat_map(move |v| iter::repeat(v).zip(self.neighbours(v)))
            .map(|(from, to)| Edge::new(from, to))
    }

    /// Return iterator over the neighbours of vertex `v`.
    ///
    /// The neighbours are all vertices `u` such that an edge from `v` to `u` exists.
    pub fn neighbours<'a>(&'a self, v: usize) -> impl Iterator<Item = usize> + 'a {
        self.lists[v].iter().cloned()
    }

    fn subrange(range: Range<usize>, from: usize, to: usize, total: usize) -> Range<usize> {
        let from = from as f32 / total as f32;
        let to = to as f32 / total as f32;

        Range {
            start: (range.start as f32 + from * range.len() as f32).floor() as usize,
            end: (range.start as f32 + to * range.len() as f32).floor() as usize,
        }
    }
}

struct DirectedPart<'a> {
    from_verts: Range<usize>,
    to_verts: Range<usize>,
    lists: &'a mut [Vec<usize>],
}

impl<'a> DirectedPart<'a> {
    fn gen(&mut self, n_edges: usize, seed: Option<<Prng as SeedableRng>::Seed>) {
        let mut added = 0;
        let mut rng = match seed {
            Some(seed) => Prng::from_seed(seed),
            None => Prng::from_entropy(),
        };

        let from_range = Uniform::new(self.v_to_i(self.from_verts.start), self.v_to_i(self.from_verts.end));
        let to_range = Uniform::new(self.to_verts.start, self.to_verts.end);

        while added < n_edges {
            let from = from_range.sample(&mut rng);
            let to = to_range.sample(&mut rng);

            if self.i_to_v(from) != to && self.lists[from].iter().find(|&&e| e == to).is_none() {
                self.lists[from].push(to);
                added += 1;
            }
        }
    }

    /// Convert from vertex id to index in this part's slice of lists.
    fn v_to_i(&self, v: usize) -> usize {
        v - self.from_verts.start
    }

    /// Convert from index in this part's slice of lists to vertex id.
    fn i_to_v(&self, i: usize) -> usize {
        i + self.from_verts.start
    }
}

#[cfg(test)]
mod tests;
