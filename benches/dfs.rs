#![feature(decl_macro)]
#![feature(test)]

extern crate parallel_dfs;
extern crate rand;
extern crate rayon;
extern crate test;

use parallel_dfs::graph::AdjLists;
use parallel_dfs::dfs;
use rand::prelude::*;
use rand::distributions::Standard;
use rand::prng::XorShiftRng;
use rayon::ThreadPoolBuilder;
use test::Bencher;

const SEED: [u8; 16] = [130, 241, 105, 144, 39, 87, 188, 11, 85, 171, 153, 10, 140, 0, 21, 127];

macro bench_par($name: ident, $t: expr, $n: expr, $m: expr) {
    #[bench]
    fn $name(bencher: &mut Bencher) {
        let mut rng = XorShiftRng::from_seed(SEED);
        let graph = AdjLists::gen_directed($n, $m, rng.sample_iter(&Standard));

        let thread_pool = ThreadPoolBuilder::new()
            .num_threads($t)
            .build()
            .unwrap();

        // from rayon docs:
        // > Executes `op` within the threadpool. Any attempts to use `join`, `scope`,
        // > or parallel iterators will then operate within that threadpool.
        thread_pool.install(|| {
            bencher.iter(|| dfs::par(&graph));
        });
    }
}

macro bench_seq($name: ident, $n: expr, $m: expr) {
    #[bench]
    fn $name(bencher: &mut Bencher) {
        let mut rng = XorShiftRng::from_seed(SEED);
        let graph = AdjLists::gen_directed($n, $m, rng.sample_iter(&Standard));

        bencher.iter(|| dfs::seq(&graph));
    }
}

bench_par!(par_t1_n4k_m400k, 1, 4_000, 400_000);
bench_par!(par_t2_n4k_m400k, 2, 4_000, 400_000);
bench_par!(par_t4_n4k_m400k, 4, 4_000, 400_000);
bench_par!(par_t8_n4k_m400k, 8, 4_000, 400_000);

bench_seq!(seq_n4k_m400k, 4_000, 400_000);
