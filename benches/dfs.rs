#![feature(decl_macro)]
#![feature(test)]

extern crate parallel_dfs;
extern crate rand;
extern crate rayon;
extern crate test;

use parallel_dfs::graph::{AdjLists, AdjMatrix};
use parallel_dfs::dfs;
use rand::prelude::*;
use rand::distributions::Standard;
use rand::prng::XorShiftRng;
use rayon::ThreadPoolBuilder;
use test::Bencher;

const SEED: [u8; 16] = [130, 241, 105, 144, 39, 87, 188, 11, 85, 171, 153, 10, 140, 0, 21, 127];

macro bench_par($name: ident, $Graph: ty, $t: expr, $n: expr, $m: expr) {
    #[bench]
    fn $name(bencher: &mut Bencher) {
        let mut rng = XorShiftRng::from_seed(SEED);
        let graph = <$Graph>::gen_directed($n, $m, rng.sample_iter(&Standard));

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

macro bench_seq($name: ident, $Graph: ty, $n: expr, $m: expr) {
    #[bench]
    fn $name(bencher: &mut Bencher) {
        let mut rng = XorShiftRng::from_seed(SEED);
        let graph = <$Graph>::gen_directed($n, $m, rng.sample_iter(&Standard));

        bencher.iter(|| dfs::seq(&graph));
    }
}

bench_seq!(seq_list_n4k_m400k, AdjLists, 4_000, 400_000);
bench_par!(par_list_t1_n4k_m400k, AdjLists, 1, 4_000, 400_000);
bench_par!(par_list_t2_n4k_m400k, AdjLists, 2, 4_000, 400_000);
bench_par!(par_list_t4_n4k_m400k, AdjLists, 4, 4_000, 400_000);
bench_par!(par_list_t6_n4k_m400k, AdjLists, 6, 4_000, 400_000);
bench_par!(par_list_t8_n4k_m400k, AdjLists, 8, 4_000, 400_000);
bench_par!(par_list_t10_n4k_m400k, AdjLists, 10, 4_000, 400_000);
bench_par!(par_list_t12_n4k_m400k, AdjLists, 12, 4_000, 400_000);
bench_par!(par_list_t14_n4k_m400k, AdjLists, 14, 4_000, 400_000);
bench_par!(par_list_t16_n4k_m400k, AdjLists, 16, 4_000, 400_000);
bench_par!(par_list_t20_n4k_m400k, AdjLists, 20, 4_000, 400_000);
bench_par!(par_list_t24_n4k_m400k, AdjLists, 24, 4_000, 400_000);
bench_par!(par_list_t28_n4k_m400k, AdjLists, 28, 4_000, 400_000);
bench_par!(par_list_t32_n4k_m400k, AdjLists, 32, 4_000, 400_000);

bench_seq!(seq_mat_n4k_m400k, AdjMatrix, 4_000, 400_000);
bench_par!(par_mat_t1_n4k_m400k, AdjMatrix, 1, 4_000, 400_000);
bench_par!(par_mat_t2_n4k_m400k, AdjMatrix, 2, 4_000, 400_000);
bench_par!(par_mat_t4_n4k_m400k, AdjMatrix, 4, 4_000, 400_000);
bench_par!(par_mat_t6_n4k_m400k, AdjMatrix, 6, 4_000, 400_000);
bench_par!(par_mat_t8_n4k_m400k, AdjMatrix, 8, 4_000, 400_000);
bench_par!(par_mat_t10_n4k_m400k, AdjMatrix, 10, 4_000, 400_000);
bench_par!(par_mat_t12_n4k_m400k, AdjMatrix, 12, 4_000, 400_000);
bench_par!(par_mat_t14_n4k_m400k, AdjMatrix, 14, 4_000, 400_000);
bench_par!(par_mat_t16_n4k_m400k, AdjMatrix, 16, 4_000, 400_000);
bench_par!(par_mat_t20_n4k_m400k, AdjMatrix, 20, 4_000, 400_000);
bench_par!(par_mat_t24_n4k_m400k, AdjMatrix, 24, 4_000, 400_000);
bench_par!(par_mat_t28_n4k_m400k, AdjMatrix, 28, 4_000, 400_000);
bench_par!(par_mat_t32_n4k_m400k, AdjMatrix, 32, 4_000, 400_000);
