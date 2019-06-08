//! Parallel DFS

use rayon::prelude::*;

use crate::graph::{AdjLists as Graph, Edge, Tree};

use std::sync::atomic::AtomicUsize;
use std::usize;

mod util;
use self::util::{take_ownership, State};

/// Perform a sequential DFS traversal of the graph and build a forest showing how it was traversed.
pub fn run(graph: &Graph) -> Vec<Tree> {
    // Tracks is a given vertex is "owned" by a tree.
    // If a tree owns the vertex, its value is the id of the root node of the tree.
    // If the vertex isn't owned, its value is `usize::MAX`.
    //
    // When a tree wants to visit a vertex it must take ownership first.
    // Ownership can be taken only once when the vertex isn't owned (value is `usize::MAX`)
    // and can't be changed afterwords.
    //
    // This is a shared state between all threads and guarantees that each vertex is traversed only once.
    let owner = (0..graph.vertices().count())
        .map(|_| AtomicUsize::new(usize::MAX))
        .collect::<Vec<_>>();

    graph
        .vertices_par()
        .filter_map(|root| {
            if !take_ownership(&owner[root], root) {
                return None;
            }

            let mut stack = Vec::new();
            let mut state = State::new(root, &owner);
            let mut tree = Tree::new(root);

            for v in graph.neighbours(root).rev() {
                if state.get(v).is_owned_unused() {
                    stack.push((root, v));
                }
            }

            while let Some((parent, v)) = stack.pop() {
                if state.get(v).is_owned_unused() {
                    state.mark_used(v);
                    tree.add(Edge::new(parent, v));

                    for child in graph.neighbours(v).rev() {
                        if state.get(child).is_owned_unused() {
                            stack.push((v, child));
                        }
                    }
                }
            }

            Some(tree)
        })
        .collect()
}

#[cfg(test)]
mod tests;
