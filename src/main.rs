extern crate parallel_dfs;
extern crate rayon;

#[macro_use]
extern crate structopt;

use parallel_dfs::dfs;
use parallel_dfs::graph::AdjLists as Graph;
use rayon::ThreadPoolBuilder;
use structopt::StructOpt;

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
            Opts::Gen { undirected: true, vertices: _, edges: _, .. } => {
                unimplemented!()
            },
            Opts::Gen { undirected: false, vertices, edges, output, .. } => {
                let graph = Graph::gen_directed(vertices, edges, None);
                let forest = dfs::par(&graph);

                if output {
                    println!("{:#?}", forest);
                }
            },
            Opts::Load { .. } => {
                unimplemented!()
            }
        }
    });
}
