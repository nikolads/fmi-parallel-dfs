use rayon::iter::ParallelIterator;

pub mod adj_lists;
pub mod adj_matrix;
pub mod tree;

pub use self::adj_lists::AdjLists;
pub use self::adj_matrix::AdjMatrix;
pub use self::tree::Tree;

/// Pseudo-random number generator algorithm used in this module.
///
/// Using XorShift because it is currently the only
/// non-cryptographically secure PRNG provided by `rand`.
type Prng = rand::prng::XorShiftRng;

/// Graph edge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
}

impl Edge {
    pub fn new(from: usize, to: usize) -> Self {
        Edge { from, to }
    }
}

pub trait GraphRef<'a> {
    type Vertices: Iterator<Item = usize> + DoubleEndedIterator + 'a;
    type VerticesPar: ParallelIterator<Item = usize> + 'a;
    type Neighbours: Iterator<Item = usize> + DoubleEndedIterator + 'a;

    fn vertices(self) -> Self::Vertices;
    fn vertices_par(self) -> Self::VerticesPar;
    fn neighbours(self, v: usize) -> Self::Neighbours;
}
