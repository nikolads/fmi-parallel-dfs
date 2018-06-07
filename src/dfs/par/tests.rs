use super::*;

#[test]
fn visits_all() {
    let graph = Graph::gen_directed(100, 1000, None);
    let forest = run(&graph);

    let mut visited = vec![0; graph.vertices().count()];

    for tree in &forest {
        visited[tree.root] += 1;

        for edge in &tree.edges {
            visited[edge.to] += 1;
        }
    }

    assert!(visited.iter().all(|&v| v == 1));
}
