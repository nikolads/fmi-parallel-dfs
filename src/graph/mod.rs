mod adj_lists;
mod tree;

pub use self::adj_lists::AdjLists;
pub use self::tree::Tree;

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

