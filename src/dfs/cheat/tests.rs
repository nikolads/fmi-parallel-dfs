use rayon::ThreadPoolBuilder;

use crate::dfs;
use crate::graph::AdjLists;

#[test]
fn visits_all() {
    let graph = AdjLists::gen_directed(100, 1000, None);
    let forest = dfs::cheat(&graph);

    let mut visited = vec![0; graph.vertices().count()];

    for tree in &forest {
        visited[tree.root] += 1;

        for edge in &tree.edges {
            visited[edge.to] += 1;
        }
    }

    assert!(visited.iter().all(|&v| v == 1));
}

// The result should be equivalent to the single threaded algorithm
// when run on 1 thread.
#[test]
fn matches_seq() {
    let thread_pool = ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .unwrap();

    thread_pool.install(|| {
        let graph = AdjLists::gen_directed(100, 1000, None);
        let mut answer = dfs::seq(&graph);
        let mut forest = dfs::cheat(&graph);

        answer.sort_unstable_by_key(|tree| tree.root);
        forest.sort_unstable_by_key(|tree| tree.root);
        assert_eq!(forest.len(), answer.len());

        for (tree, answer) in forest.iter_mut().zip(&mut answer) {
            tree.edges.sort_unstable_by_key(|edge| edge.from);
            answer.edges.sort_unstable_by_key(|edge| edge.from);

            assert_eq!(tree.root, answer.root);
            assert_eq!(tree.edges, answer.edges);
        }
    });
}
