use rayon::ThreadPoolBuilder;
use super::*;

#[test]
fn generate_directed() {
    ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .unwrap()
        .install(|| {
            let graph = AdjMatrix::gen_directed(30, 100, None);
            assert_eq!(graph.vertices().count(), 30);
            assert_eq!(graph.edges().count(), 100);
        });

    let graph = AdjMatrix::gen_directed(30, 100, None);
    assert_eq!(graph.vertices().count(), 30);
    assert_eq!(graph.edges().count(), 100);

    let graph = AdjMatrix::gen_directed(300, 10000, None);
    assert_eq!(graph.vertices().count(), 300);
    assert_eq!(graph.edges().count(), 10000);
}

#[test]
fn generate_directed_threads() {
    let graph = AdjMatrix::gen_directed_on_threads(30, 100, 1, None);
    assert_eq!(graph.vertices().count(), 30);
    assert_eq!(graph.edges().count(), 100);

    let graph = AdjMatrix::gen_directed_on_threads(30, 100, 4, None);
    assert_eq!(graph.vertices().count(), 30);
    assert_eq!(graph.edges().count(), 100);
}

#[test]
fn generate_undirected() {
    ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .unwrap()
        .install(|| {
            let graph = AdjMatrix::gen_undirected(30, 100, None);
            assert_eq!(graph.vertices().count(), 30);
            assert_eq!(graph.edges().count(), 200);
        });

    let graph = AdjMatrix::gen_undirected(30, 100, None);
    assert_eq!(graph.vertices().count(), 30);
    assert_eq!(graph.edges().count(), 200);

    let graph = AdjMatrix::gen_undirected(300, 10000, None);
    assert_eq!(graph.vertices().count(), 300);
    assert_eq!(graph.edges().count(), 20000);
}
