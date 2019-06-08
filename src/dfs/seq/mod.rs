//! Sequential DFS

use crate::graph::{AdjLists as Graph, Edge, Tree};

/// Perform a sequential DFS traversal of the graph and build a forest showing how it was traversed.
pub fn run(graph: &Graph) -> Vec<Tree> {
    let mut result = Vec::new();
    let mut used = vec![false; graph.vertices().count()];
    let mut stack = Vec::new();

    for root in graph.vertices() {
        if used[root] {
            continue;
        }

        let mut tree = Tree::new(root);
        used[root] = true;

        for v in graph.neighbours(root).rev() {
            if !used[v] {
                stack.push((root, v));
            }
        }

        while !stack.is_empty() {
            let (parent, vert) = stack.pop().unwrap();

            if !used[vert] {
                used[vert] = true;
                tree.add(Edge::new(parent, vert));

                for child in graph.neighbours(vert).rev() {
                    if !used[child] {
                        stack.push((vert, child));
                    }
                }
            }
        }

        result.push(tree);
    }

    result
}
