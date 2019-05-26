use rayon::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::graph::{AdjMatrix, Edge, Tree};

pub fn run(graph: &AdjMatrix) -> Vec<Tree> {
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

            let (mut descend_tree, mut backtrack_stack) = descend(graph, &owner, root as u32, backtrack_start_index);

            backtrack_stack.pop();
            backtrack_start_index += 1;

            let mut backtrack = backtrack(graph, &owner, &backtrack_stack, backtrack_start_index);

            backtrack
                .par_iter_mut()
                .for_each(|(backtrack_index, ref mut tree)| {
                    tree.edges
                        .retain(|edge| owner[edge.to].load(Ordering::Relaxed) == *backtrack_index)
                });

            backtrack.into_iter().for_each(|(_, tree)| {
                descend_tree.edges.extend(tree.edges);
            });


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

fn descend(graph: &AdjMatrix, owner: &[AtomicU32], root: u32, backtrack_start_index: usize) -> (Tree, Vec<u32>) {
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

fn backtrack(
    graph: &AdjMatrix,
    owner: &[AtomicU32],
    backtrack_stack: &[u32],
    backtrack_start_index: usize,
) -> Vec<(u32, Tree)> {
    let n_verts = graph.vertices().count();

    backtrack_stack
        .par_iter()
        .rev()
        .enumerate()
        .filter_map(|(backtrack_index, &node)| {
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

            match tree.edges.is_empty() {
                true => None,
                false => Some((backtrack_index, tree)),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests;
