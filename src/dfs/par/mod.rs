use rayon::prelude::*;

use graph::{AdjLists as Graph, Edge, Tree};

use std::sync::atomic::AtomicUsize;
use std::usize;

mod util;
use self::util::{take_ownership, State};

pub fn run(graph: &Graph) -> Vec<Tree> {
    // can't use `vec![AtomicUsize::new(usize::MAX); graph.num_vertices()]`
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

            for v in graph.neighbours(root) {
                if state.get(v).is_owned_unused() {
                    stack.push((root, v));
                }
            }

            while let Some((parent, v)) = stack.pop() {
                if state.get(v).is_owned_unused() {
                    state.mark_used(v);
                    tree.add(Edge::new(parent, v));

                    for child in graph.neighbours(v) {
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
