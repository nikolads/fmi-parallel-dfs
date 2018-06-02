use rand;
use rand::distributions::Uniform;
use rand::prelude::*;
use rayon;
use rayon::prelude::*;

use std::iter;
use std::mem;
use std::ops::Range;

/// Pseudo-random number generator algorithm used in this module.
///
/// Using XorShift because it is currently the only
/// non-cryptographically secure PRNG provided by `rand`.
pub type Prng = rand::prng::XorShiftRng;

#[derive(Debug, Clone)]
pub struct AdjLists {
    n_verts: usize,
    lists: Vec<Vec<usize>>,
}

impl AdjLists {
    pub fn new(n_verts: usize) -> Self {
        AdjLists {
            n_verts,
            lists: vec![vec![]; n_verts],
        }
    }

    pub fn gen_directed<I>(n_verts: usize, n_edges: usize, n_threads: usize, seeds: I) -> Self
    where
        I: IntoIterator<Item = <Prng as SeedableRng>::Seed>,
        <I as IntoIterator>::IntoIter: Clone + Send,
    {
        assert!(n_edges <= n_verts * (n_verts - 1));

        let mut graph = AdjLists::new(n_verts);

        let mut state = &mut graph.lists[..];
        let mut seeds = seeds.into_iter().cycle();

        rayon::scope(|scope| {
            for i in 0..n_threads {
                let verts_per_thread = (1.0 / n_threads as f32) * n_verts as f32;

                let from_verts = Range {
                    start: (i as f32 * verts_per_thread).floor() as usize,
                    end: ((i + 1) as f32 * verts_per_thread).floor() as usize,
                };

                let tmp = mem::replace(&mut state, &mut []);
                let (lists, next) = tmp.split_at_mut(from_verts.end - from_verts.start);
                state = next;

                let mut part = DirectedPart {
                    from_verts: from_verts,
                    to_verts: 0..n_verts,
                    lists: lists,
                };

                let edges_per_thread = (1.0 / n_threads as f32) * n_edges as f32;
                let edges = ((i + 1) as f32 * edges_per_thread).floor() as usize -
                    (i as f32 * edges_per_thread).floor() as usize;

                let seed = seeds.next();

                scope.spawn(move |_| part.gen(edges, seed));
            }
        });

        graph
    }

    pub fn sort(&mut self) {
        self.lists.par_iter_mut().for_each(|list| list.sort_unstable())
    }

    /// Returns iterator over all vertices in the graph.
    pub fn vertices<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        0..self.n_verts
    }

    /// Returns iterator over all edges in the graph.
    pub fn edges<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        self.vertices()
            .flat_map(move |v| iter::repeat(v).zip(self.neighbours(v)))
    }

    /// Return iterator over the neighbours of vertex `v`.
    ///
    /// The neighbours are all vertices `u` such that an edge from `v` to `u` exists.
    pub fn neighbours<'a>(&'a self, v: usize) -> impl Iterator<Item = usize> + 'a {
        self.lists[v].iter().cloned()
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

        let from_range = Uniform::new(0, self.from_verts.end - self.from_verts.start);
        let to_range = Uniform::new(self.to_verts.start, self.to_verts.end);

        while added < n_edges {
            let from = from_range.sample(&mut rng);
            let to = to_range.sample(&mut rng);

            if from != to && self.lists[from].iter().find(|&&e| e == to).is_none() {
                self.lists[from].push(to);
                added += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests;
