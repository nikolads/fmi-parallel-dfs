#![feature(nll)]

extern crate rand;
extern crate rayon;

pub mod graph;
pub mod dfs;

use graph::AdjLists;

fn main() {
    let mut graph = AdjLists::gen_directed(10, 30, None);
    graph.sort();
    println!("{:?}", graph);
}
