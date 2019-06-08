extern crate parallel_dfs;
extern crate rayon;
extern crate structopt;

use parallel_dfs::dfs;
use parallel_dfs::graph::{AdjLists, AdjMatrix};
use rayon::ThreadPoolBuilder;
use structopt::StructOpt;
use std::str::FromStr;

enum Algorithm {
    Seq,
    ParMatrix
}

impl FromStr for Algorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seq" => Ok(Algorithm::Seq),
            "par_mat" => Ok(Algorithm::ParMatrix),
            _ => Err("".to_string())
        }
    }
}

#[allow(dead_code)]
#[derive(StructOpt)]
enum Opts {
    /// Generate a random graph
    #[structopt(name = "gen")]
    Gen {
        /// Generate indirected graph. Defaults to directed.
        #[structopt(long = "undirected")]
        undirected: bool,
        /// Number of vertices to generate.
        #[structopt(short = "n", long = "vertices")]
        vertices: usize,
        /// Number of edges to generate.
        #[structopt(short = "m", long = "edges")]
        edges: usize,
        /// Number of threads to use. Defaults to number of logical CPUs.
        #[structopt(short = "t", long = "threads")]
        threads: Option<usize>,
        #[structopt(long = "algo")]
        algorithm: Option<Algorithm>,
        /// Whether to write the result to stdout.
        #[structopt(long = "output")]
        output: bool,
    },
    /// Load graph from file
    #[structopt(name = "load")]
    Load {
        /// File to load.
        #[structopt(short = "f", long = "file")]
        file: String,
        /// Number of threads to use. Defaults to number of logical CPUs.
        #[structopt(short = "t", long = "threads")]
        threads: Option<usize>,
        /// Whether to write the result to stdout.
        #[structopt(long = "output")]
        output: bool,
    },
}

fn main() {
    let opts = Opts::from_args();

    let thread_pool = match opts {
        Opts::Gen { threads: Some(t), .. } | Opts::Load { threads: Some(t), .. } => {
            ThreadPoolBuilder::new().num_threads(t).build().unwrap()
        },
        _ => {
            ThreadPoolBuilder::new().build().unwrap()
        }
    };

    thread_pool.install(|| {
        match opts {
            Opts::Gen { undirected: true, vertices, edges, output, threads, .. } => {
                let graph = AdjLists::gen_undirected(vertices, edges, None);
                // let forest = dfs::par(&graph);

                // if output {
                //     println!("{:#?}", forest);
                // }
            },
            Opts::Gen { undirected: false, vertices, edges, output, algorithm, .. } => {
                let start;

                let forest = match algorithm.unwrap_or(Algorithm::ParMatrix) {
                    Algorithm::Seq => {
                        let graph = AdjLists::gen_directed(vertices, edges, None);
                        start = std::time::Instant::now();
                        dfs::seq(&graph)
                    }
                    Algorithm::ParMatrix => {
                        let before_gen = std::time::Instant::now();

                        let graph = AdjMatrix::gen_directed(vertices, edges, None);

                        start = std::time::Instant::now();
                        println!("matrix gen: {:?}", start.duration_since(before_gen));

                        dfs::par_matrix(&graph)
                    }
                };

                if output {
                    println!("{:#?}", forest);
                    // println!("{:#?}", forest.iter().map(|tree| tree.edges.len()).collect::<Vec<_>>());
                }

                println!("Total run time: {:?}", std::time::Instant::now().duration_since(start));
            },
            Opts::Load { .. } => {
                unimplemented!()
            }
        }
    });
}
