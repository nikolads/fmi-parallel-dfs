use rand::distributions::Uniform;
use rand::prelude::*;
use rayon::{self, prelude::*};
use std::iter;

use crate::graph::{Edge, GraphRef, Prng};
use crate::utils::BitVec;

#[derive(Debug)]
pub struct AdjMatrix {
    n_verts: usize,
    data: BitVec,
}

impl AdjMatrix {
    /// Create new empty graph
    pub fn new(n_verts: usize) -> Self {
        let start = std::time::Instant::now();

        let graph = Self {
            n_verts,
            data: BitVec::new(n_verts * n_verts),
        };

        let after_alloc = std::time::Instant::now();
        println!("    graph alloc: {:?}", after_alloc.duration_since(start));

        graph
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

        let graph = Self::new(n_verts);
        const EDGES_PER_CHUNK: usize = 128;

        let chunks = if n_edges % EDGES_PER_CHUNK == 0 {
            n_edges / EDGES_PER_CHUNK
        } else {
            n_edges / EDGES_PER_CHUNK + 1
        };

        // Calculate the number of seeds that we will need and pre-collect them in a
        // vector. We need this because we can't share a mutable iterator
        // between threads without locking.
        let seeds = seeds
            .into_iter()
            .map(|s| Some(s))
            .chain(iter::repeat(None))
            .take(chunks)
            .collect::<Vec<_>>();

        seeds.into_par_iter().enumerate().for_each(|(i, seed)| {
            let edges_to_gen = if i < chunks - 1 {
                EDGES_PER_CHUNK
            } else {
                n_edges % EDGES_PER_CHUNK
            };

            let mut added = 0;
            let mut rng = match seed {
                Some(seed) => Prng::from_seed(seed),
                None => Prng::from_entropy(),
            };

            let range = Uniform::new(0, n_verts);

            while added < edges_to_gen {
                let from = range.sample(&mut rng);
                let to = range.sample(&mut rng);

                if graph.should_add(from, to) {
                    if graph.data.swap(graph.index(from, to), true) == false {
                        added += 1;
                    }
                }
            }
        });

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

        let graph = Self::new(n_verts);
        let seeds = seeds.into_iter().map(|s| Some(s)).chain(iter::repeat(None));

        let edges_per_thread = if n_edges % n_threads == 0 {
            n_edges / n_threads
        } else {
            n_edges / n_threads + 1
        };

        rayon::scope(|scope| {
            (0..n_threads).zip(seeds).for_each(|(t, seed)| {
                let graph = &graph;

                scope.spawn(move |_| {
                    let edges_to_gen = if t < n_threads - 1 || n_edges % n_threads == 0 {
                        edges_per_thread
                    } else {
                        n_edges % edges_per_thread
                    };

                    let mut added = 0;
                    let mut rng = match seed {
                        Some(seed) => Prng::from_seed(seed),
                        None => Prng::from_entropy(),
                    };

                    let range = Uniform::new(0, n_verts);

                    while added < edges_to_gen {
                        let from = range.sample(&mut rng);
                        let to = range.sample(&mut rng);

                        if graph.should_add(from, to) {
                            if graph.data.swap(graph.index(from, to), true) == false {
                                added += 1;
                            }
                        }
                    }
                });
            });
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

        let graph = Self::new(n_verts);
        const EDGES_PER_CHUNK: usize = 128;

        let chunks = (0..n_edges).step_by(EDGES_PER_CHUNK).count();

        // Calculate the number of seeds that we will need and pre-collect them in a
        // vector. We need this because we can't share a mutable iterator
        // between threads without locking.
        let seeds = seeds
            .into_iter()
            .map(|s| Some(s))
            .chain(iter::repeat(None))
            .take(chunks)
            .collect::<Vec<_>>();

        seeds.into_par_iter().enumerate().for_each(|(i, seed)| {
            let edges_to_gen = if i < chunks - 1 {
                EDGES_PER_CHUNK
            } else {
                n_edges % EDGES_PER_CHUNK
            };

            let mut added = 0;
            let mut rng = match seed {
                Some(seed) => Prng::from_seed(seed),
                None => Prng::from_entropy(),
            };

            let range = Uniform::new(1, n_verts);

            while added < edges_to_gen {
                let to = range.sample(&mut rng);
                let from = rng.gen_range(0, to);

                if graph.should_add(from, to) {
                    if graph.data.swap(graph.index(from, to), true) == false {
                        graph.data.set(graph.index(to, from), true);
                        added += 1;
                    }
                }
            }
        });

        graph
    }

    fn should_add(&self, from: usize, to: usize) -> bool {
        from != to && self.data.get(self.index(from, to)).unwrap() == false
    }

    /// Get index in the one dimentional array for the element at `row` and `col`.
    fn index(&self, row: usize, col: usize) -> usize {
        row * self.n_verts + col
    }

    /// Iterator over all edges in the graph.
    pub fn edges<'a>(&'a self) -> impl Iterator<Item = Edge> + 'a {
        self.vertices()
            .flat_map(move |v| iter::repeat(v).zip(self.neighbours(v)))
            .map(|(from, to)| Edge::new(from, to))
    }
}

impl<'a> GraphRef<'a> for &'a AdjMatrix {
    type Vertices = std::ops::Range<usize>;
    type VerticesPar = rayon::range::Iter<usize>;
    type Neighbours = crate::utils::bit_vec::Ones<'a>;

    /// Iterator over all vertices in the graph.
    fn vertices(self) -> Self::Vertices {
        0..self.n_verts
    }

    /// Rayon parallel iterator over all vertices in the graph.
    fn vertices_par(self) -> Self::VerticesPar {
        (0..self.n_verts).into_par_iter()
    }

    /// Iterator over the neighbours of vertex `v`.
    ///
    /// The neighbours are all vertices `u` such that an edge from `v` to `u`
    /// exists.
    fn neighbours(self, v: usize) -> Self::Neighbours {
        let start = v * self.n_verts;
        let end = (v + 1) * self.n_verts;
        self.data.slice(start..end).ones()
    }
}

#[cfg(test)]
mod tests;
