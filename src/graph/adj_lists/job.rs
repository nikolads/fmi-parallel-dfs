use rand::distributions::Uniform;
use rand::prelude::*;
use rayon::prelude::*;
use std::mem;
use std::ops::Range;

use super::Prng;

pub struct Job<'a> {
    from_verts: Range<usize>,
    to_verts: Range<usize>,
    n_edges: usize,
    lists: &'a mut [Vec<usize>],
    directed: bool,
}

impl<'a> Job<'a> {
    pub fn gen(&mut self, seed: Option<<Prng as SeedableRng>::Seed>) {
        let mut added = 0;
        let mut rng = match seed {
            Some(seed) => Prng::from_seed(seed),
            None => Prng::from_entropy(),
        };

        let from_range = Uniform::new(self.from_verts.start, self.from_verts.end);
        let to_range = Uniform::new(self.to_verts.start, self.to_verts.end);

        while added < self.n_edges {
            let from = from_range.sample(&mut rng);
            let to = to_range.sample(&mut rng);

            if self.should_add(from, to) {
                self.list_at(from).push(to);
                added += 1;
            }
        }
    }

    fn should_add(&mut self, from: usize, to: usize) -> bool {
        if self.directed {
            from != to && self.list_at(from).iter().find(|&&e| e == to).is_none()
        } else {
            from > to && self.list_at(from).iter().find(|&&e| e == to).is_none()
        }
    }

    fn list_at(&mut self, v: usize) -> &mut Vec<usize> {
        &mut self.lists[v - self.from_verts.start]
    }
}

#[derive(Debug)]
pub struct JobDesc<'a> {
    pub n_verts: usize,
    pub n_edges: usize,
    pub lists: &'a mut [Vec<usize>],
    pub directed: bool,
}

impl<'a> JobDesc<'a> {
    pub fn chunked(self, verts_per_chunk: usize) -> impl IndexedParallelIterator<Item = Job<'a>> + 'a{
        let directed = self.directed;
        let n_edges = self.n_edges;
        let n_verts = self.n_verts;

        self.lists
            .par_chunks_mut(verts_per_chunk)
            .enumerate()
            .map(move |(i, lists)| {
                let start = i * verts_per_chunk;
                let end = start + lists.len();

                if directed {
                    let from_verts = start..end;
                    let to_verts = 0..n_verts;
                    let n_edges = Self::edges_count_directed(from_verts.clone(), n_verts, n_edges);

                    Job {
                        from_verts,
                        to_verts,
                        n_edges,
                        lists,
                        directed: true,
                    }
                } else {
                    let from_verts = start..end;
                    let to_verts = 0..end;
                    let n_edges = Self::edges_count_undirected(from_verts.clone(), n_verts, n_edges);

                    Job {
                        from_verts,
                        to_verts,
                        n_edges,
                        lists: lists,
                        directed: false,
                    }
                }
            })
    }

    pub fn threaded(self, n_threads: usize) -> impl Iterator<Item = Job<'a>> {
        let n_verts = self.n_verts;
        let n_edges = self.n_edges;
        let mut state = self.lists;

        (0..n_threads).map(move |i| {
            let from_verts = Self::subrange(0..n_verts, i, n_threads);

            // `mem::replace` to "trick" the borrow checker
            let tmp = mem::replace(&mut state, &mut []);
            let (lists, next) = tmp.split_at_mut(from_verts.end - from_verts.start);
            state = next;

            let n_edges = Self::subrange(0..n_edges, i, n_threads).len();

            Job {
                from_verts,
                to_verts: 0..n_verts,
                n_edges,
                lists,
                directed: true,
            }
        })
    }

    /// Number of edges this job must generate for a directed graph.
    #[inline]
    fn edges_count_directed(from_verts: Range<usize>, n_verts: usize, n_edges: usize) -> usize {
        let from = from_verts.start as f64 / n_verts as f64;
        let to = from_verts.end as f64 / n_verts as f64;

        (to * n_edges as f64).floor() as usize - (from * n_edges as f64).floor() as usize
    }

    /// Number of edges this job must generate for an undirected graph.
    #[inline]
    fn edges_count_undirected(from_verts: Range<usize>, n_verts: usize, n_edges: usize) -> usize {
        let from = from_verts.start as f64 / n_verts as f64;
        let to = from_verts.end as f64 / n_verts as f64;

        (to * to * n_edges as f64).floor() as usize -
            (from * from * n_edges as f64).floor() as usize
    }

    fn subrange(range: Range<usize>, t: usize, n_threads: usize) -> Range<usize> {
        let from = t as f32 / n_threads as f32;
        let to = (t + 1) as f32 / n_threads as f32;

        Range {
            start: (range.start as f32 + from * range.len() as f32).floor() as usize,
            end: (range.start as f32 + to * range.len() as f32).floor() as usize,
        }
    }
}
