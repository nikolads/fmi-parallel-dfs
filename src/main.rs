#![feature(nll)]

extern crate rand;
extern crate rayon;

pub mod graph;

use graph::AdjLists;

fn main() {
    let mut graph = AdjLists::gen_directed(10, 30, 4, None);
    graph.sort();
    println!("{:?}", graph);
}
