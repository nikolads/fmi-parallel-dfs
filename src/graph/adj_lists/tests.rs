use super::*;

#[test]
fn generate_directed() {
    let graph = AdjLists::gen_directed(30, 100, None);
    assert_eq!(graph.vertices().count(), 30);
    assert_eq!(graph.edges().count(), 100);

    let graph = AdjLists::gen_directed(300, 10000, None);
    assert_eq!(graph.vertices().count(), 300);
    assert_eq!(graph.edges().count(), 10000);
}


#[test]
fn generate_directed_threads() {
    let graph = AdjLists::gen_directed_on_threads(30, 100, 1, None);
    assert_eq!(graph.vertices().count(), 30);
    assert_eq!(graph.edges().count(), 100);

    let graph = AdjLists::gen_directed_on_threads(30, 100, 4, None);
    assert_eq!(graph.vertices().count(), 30);
    assert_eq!(graph.edges().count(), 100);
}
