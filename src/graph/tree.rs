use graph::Edge;

/// Simple tree representation.
///
/// Represents a tree by the root node and a flat list of tree edges.
///
#[derive(Debug, Clone)]
pub struct Tree {
    pub root: usize,
    pub edges: Vec<Edge>,
}

impl Tree {
    pub fn new(root: usize) -> Self {
        Tree { root, edges: vec![] }
    }

    pub fn add(&mut self, edge: Edge) {
        self.edges.push(edge);
    }
}
