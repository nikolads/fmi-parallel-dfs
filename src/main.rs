use parallel_dfs::dfs;
use parallel_dfs::graph::{AdjLists, AdjMatrix};
use rayon::ThreadPoolBuilder;
use structopt::StructOpt;
use std::str::FromStr;

enum Algorithm {
    GenList,
    SeqList,
    ParList,
    CheatList,
    GenMatrix,
    SeqMatrix,
    ParMatrix,
    CheatMatrix,
}

impl FromStr for Algorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gen_list" => Ok(Algorithm::GenList),
            "seq_list" => Ok(Algorithm::SeqList),
            "par_list" => Ok(Algorithm::ParList),
            "cheat_list" => Ok(Algorithm::CheatList),
            "gen_mat" => Ok(Algorithm::GenMatrix),
            "seq_mat" => Ok(Algorithm::SeqMatrix),
            "par_mat" => Ok(Algorithm::ParMatrix),
            "cheat_mat" => Ok(Algorithm::CheatMatrix),
            _ => Err(format!("unknown algorithm {:?}", s))
        }
    }
}

#[allow(dead_code)]
#[derive(StructOpt)]
enum Opts {
    /// Generate a random graph
    #[structopt(name = "gen")]
    Gen {
        /// Number of vertices to generate.
        #[structopt(short = "n", long = "vertices")]
        vertices: usize,
        /// Number of edges to generate.
        #[structopt(short = "m", long = "edges")]
        edges: usize,
        /// Number of threads to use. Defaults to number of logical CPUs.
        #[structopt(short = "t", long = "threads")]
        threads: Option<usize>,
        /// Which algorithm to use.
        #[structopt(long = "algo")]
        algorithm: Option<Algorithm>,
        /// Generate undirected graph. Defaults to directed.
        #[structopt(long = "undirected")]
        undirected: bool,
        /// Whether to write the result to stdout.
        #[structopt(long = "output")]
        output: bool,
    },
}

fn main() {
    let opts = Opts::from_args();

    // Manually build the global thread pool so we can set the number
    // of threads to use
    let thread_pool = match opts {
        Opts::Gen { threads: Some(t), .. } => {
            ThreadPoolBuilder::new().num_threads(t).build().unwrap()
        },
        _ => {
            ThreadPoolBuilder::new().build().unwrap()
        }
    };

    thread_pool.install(|| {
        match opts {
            Opts::Gen { undirected, vertices, edges, output, algorithm, .. } => {
                let algorithm = algorithm.unwrap_or(Algorithm::ParMatrix);
                let forest = match algorithm {
                    Algorithm::GenList => {
                        let start = std::time::Instant::now();
                        let _graph = match undirected {
                            true => AdjLists::gen_undirected(vertices, edges, None),
                            false => AdjLists::gen_directed(vertices, edges, None),
                        };

                        let after_gen = std::time::Instant::now();
                        println!("graph gen: {:?}", after_gen.duration_since(start));

                        vec![]
                    },
                    Algorithm::GenMatrix => {
                        let start = std::time::Instant::now();
                        let _graph = match undirected {
                            true => AdjMatrix::gen_undirected(vertices, edges, None),
                            false => AdjMatrix::gen_directed(vertices, edges, None),
                        };

                        let after_gen = std::time::Instant::now();
                        println!("graph gen: {:?}", after_gen.duration_since(start));

                        vec![]
                    }
                    Algorithm::SeqList | Algorithm::ParList | Algorithm::CheatList => {
                        let start = std::time::Instant::now();
                        let graph = match undirected {
                            true => AdjLists::gen_undirected(vertices, edges, None),
                            false => AdjLists::gen_directed(vertices, edges, None),
                        };

                        let after_gen = std::time::Instant::now();
                        println!("graph gen: {:?}", after_gen.duration_since(start));

                        let forest = match algorithm {
                            Algorithm::SeqList => dfs::seq(&graph),
                            Algorithm::ParList => dfs::par(&graph),
                            Algorithm::CheatList => dfs::cheat(&graph),
                            _ => unreachable!()
                        };
                        println!("total dfs: {:?}", std::time::Instant::now().duration_since(after_gen));

                        forest
                    },
                    Algorithm::SeqMatrix | Algorithm::ParMatrix | Algorithm::CheatMatrix => {
                        let start = std::time::Instant::now();
                        let graph = match undirected {
                            true => AdjMatrix::gen_undirected(vertices, edges, None),
                            false => AdjMatrix::gen_directed(vertices, edges, None),
                        };

                        let after_gen = std::time::Instant::now();
                        println!("graph gen: {:?}", after_gen.duration_since(start));

                        let forest = match algorithm {
                            Algorithm::SeqMatrix => dfs::seq(&graph),
                            Algorithm::ParMatrix => dfs::par(&graph),
                            Algorithm::CheatMatrix => dfs::cheat(&graph),
                            _ => unreachable!()
                        };
                        println!("total dfs: {:?}", std::time::Instant::now().duration_since(after_gen));

                        forest
                    },
                };

                if output {
                    println!("{:#?}", forest);
                }
            },
        }
    });
}
