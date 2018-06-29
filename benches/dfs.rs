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

bench_seq!(seq_n4k_m400k, 4_000, 400_000);
bench_par!(par_t1_n4k_m400k, 1, 4_000, 400_000);
bench_par!(par_t2_n4k_m400k, 2, 4_000, 400_000);
bench_par!(par_t4_n4k_m400k, 4, 4_000, 400_000);
bench_par!(par_t6_n4k_m400k, 6, 4_000, 400_000);
bench_par!(par_t8_n4k_m400k, 8, 4_000, 400_000);
bench_par!(par_t10_n4k_m400k, 10, 4_000, 400_000);
bench_par!(par_t12_n4k_m400k, 12, 4_000, 400_000);
bench_par!(par_t14_n4k_m400k, 14, 4_000, 400_000);
bench_par!(par_t16_n4k_m400k, 16, 4_000, 400_000);
bench_par!(par_t20_n4k_m400k, 20, 4_000, 400_000);
bench_par!(par_t24_n4k_m400k, 24, 4_000, 400_000);
bench_par!(par_t28_n4k_m400k, 28, 4_000, 400_000);
bench_par!(par_t32_n4k_m400k, 32, 4_000, 400_000);

bench_seq!(seq_n40k_m400k, 40_000, 400_000);
bench_par!(par_t1_n40k_m400k, 1, 40_000, 400_000);
bench_par!(par_t4_n40k_m400k, 4, 40_000, 400_000);
bench_par!(par_t8_n40k_m400k, 8, 40_000, 400_000);
bench_par!(par_t16_n40k_m400k, 16, 40_000, 400_000);
bench_par!(par_t32_n40k_m400k, 32, 40_000, 400_000);

bench_seq!(seq_n4k_m4m, 4_000, 4_000_000);
bench_par!(par_t1_n4k_m4m, 1, 4_000, 4_000_000);
bench_par!(par_t4_n4k_m4m, 4, 4_000, 4_000_000);
bench_par!(par_t8_n4k_m4m, 8, 4_000, 4_000_000);
bench_par!(par_t16_n4k_m4m, 16, 4_000, 4_000_000);
bench_par!(par_t32_n4k_m4m, 32, 4_000, 4_000_000);
