use rayon::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::graph::{Edge, GraphRef, Tree};

pub fn run<'a, G: GraphRef<'a> + Copy + Send + Sync>(graph: G) -> Vec<Tree> {
    const NOT_VISITED: u32 = u32::max_value();

    let n_verts = graph.vertices().count();
    assert!(n_verts < u32::max_value() as usize);

    let mut owner = Vec::with_capacity(n_verts);
    owner.resize_with(n_verts, || AtomicU32::new(NOT_VISITED));

    let mut backtrack_start_index = 0;

    graph
        .vertices()
        .filter(|&root| owner[root].load(Ordering::Relaxed) == NOT_VISITED)
        .map(|root| {
            assert!(take(&owner[root], backtrack_start_index as u32));

            let start = std::time::Instant::now();

            let (mut descend_tree, mut backtrack_stack) =
                descend(graph, &owner, root as u32, backtrack_start_index);

            let after_descend = std::time::Instant::now();
            println!("    descend: {:?} (depth: {})", after_descend.duration_since(start), backtrack_stack.len());

            backtrack_stack.pop();
            backtrack_start_index += 1;

            let mut backtrack = backtrack(graph, &owner, &backtrack_stack, backtrack_start_index);

            let after_backtrack = std::time::Instant::now();
            println!("    backtrack: {:?}", after_backtrack.duration_since(after_descend));

            backtrack
                .par_iter_mut()
                .for_each(|(backtrack_index, ref mut tree)| {
                    tree.edges
                        .retain(|edge| owner[edge.to].load(Ordering::Relaxed) == *backtrack_index)
                });


            backtrack.into_iter().for_each(|(_, tree)| {
                descend_tree.edges.extend(tree.edges);
            });

            let after_post_process = std::time::Instant::now();
            println!("    post process: {:?}", after_post_process.duration_since(after_backtrack));

            backtrack_start_index += backtrack_stack.len();
            descend_tree
        })
        .collect()
}

fn take(owner: &AtomicU32, new: u32) -> bool {
    owner
        .fetch_update(
            |found| match found > new {
                true => Some(new),
                false => None,
            },
            Ordering::Relaxed,
            Ordering::Relaxed,
        )
        .is_ok()
}

fn descend<'a, G: GraphRef<'a> + Copy + Sync>(
    graph: G,
    owner: &[AtomicU32],
    root: u32,
    backtrack_start_index: usize,
) -> (Tree, Vec<u32>) {
    let n_verts = graph.vertices().count();

    let mut tree = Tree::new(root as usize);
    let mut used = vec![false; n_verts];
    let mut backtrack_stack = Vec::with_capacity(n_verts);

    let mut parent = root;

    while let Some(child) = graph
        .neighbours(parent as usize)
        .filter(|&v| !used[v] && take(&owner[v], backtrack_start_index as u32))
        .next()
    {
        used[child] = true;
        tree.add(Edge::new(parent as usize, child));
        backtrack_stack.push(child as u32);

        parent = child as u32;
    }

    (tree, backtrack_stack)
}

fn backtrack<'a, G: GraphRef<'a> + Copy + Send + Sync>(
    graph: G,
    owner: &[AtomicU32],
    backtrack_stack: &[u32],
    backtrack_start_index: usize,
) -> Vec<(u32, Tree)> {
    let n_verts = graph.vertices().count();
    let mut result = vec![(0, Tree::new(0)); backtrack_stack.len()];

    rayon::scope(|scope| {
        backtrack_stack
            .iter()
            .rev()
            .enumerate()
            .zip(&mut result)
            .for_each(|((backtrack_index, &node), out)| {
                scope.spawn(move |_| {
                    let backtrack_index = (backtrack_index + backtrack_start_index) as u32;

                    let mut used = vec![false; n_verts];
                    let mut stack = Vec::new();
                    let mut tree = Tree::new(node as usize);

                    graph
                        .neighbours(node as usize)
                        .rev()
                        .filter(|&v| owner[v].load(Ordering::Relaxed) >= backtrack_index)
                        .for_each(|v| stack.push((node, v)));

                    while let Some((parent, child)) = stack.pop() {
                        if used[child as usize] || !take(&owner[child as usize], backtrack_index) {
                            continue;
                        }

                        used[child as usize] = true;
                        tree.add(Edge::new(parent as usize, child as usize));

                        graph
                            .neighbours(child as usize)
                            .rev()
                            .filter(|&v| owner[v].load(Ordering::Relaxed) >= backtrack_index)
                            .for_each(|v| stack.push((child as u32, v)));
                    }

                    *out = (backtrack_index, tree);
                })
            });
    });

    result
}

#[cfg(test)]
mod tests;
