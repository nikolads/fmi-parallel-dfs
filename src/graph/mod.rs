mod adj_lists;
mod adj_matrix;
mod tree;

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

