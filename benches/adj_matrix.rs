#![feature(decl_macro)]
#![feature(test)]

extern crate parallel_dfs;
extern crate rayon;
extern crate test;

use parallel_dfs::graph::AdjMatrix;
use rayon::ThreadPoolBuilder;
use test::Bencher;

macro bench_rayon($name: ident, $t: expr, $n: expr, $m: expr) {
    #[bench]
    fn $name(bencher: &mut Bencher) {
        let thread_pool = ThreadPoolBuilder::new()
            .num_threads($t)
            .build()
            .unwrap();

        // from rayon docs:
        // > Executes `op` within the threadpool. Any attempts to use `join`, `scope`,
        // > or parallel iterators will then operate within that threadpool.
        thread_pool.install(|| {
            bencher.iter(|| AdjMatrix::gen_directed($n, $m, None))
        });
    }
}

macro bench_custom($name: ident, $t: expr, $n: expr, $m: expr) {
    #[bench]
    fn $name(bencher: &mut Bencher) {
        bencher.iter(|| AdjMatrix::gen_directed_on_threads($n, $m, $t, None))
    }
}

bench_rayon!(rayon_t1_n4k_m400k, 1, 4_000, 400_000);
bench_rayon!(rayon_t2_n4k_m400k, 2, 4_000, 400_000);
bench_rayon!(rayon_t4_n4k_m400k, 4, 4_000, 400_000);
bench_rayon!(rayon_t6_n4k_m400k, 6, 4_000, 400_000);
bench_rayon!(rayon_t8_n4k_m400k, 8, 4_000, 400_000);
bench_rayon!(rayon_t10_n4k_m400k, 10, 4_000, 400_000);
bench_rayon!(rayon_t12_n4k_m400k, 12, 4_000, 400_000);
bench_rayon!(rayon_t14_n4k_m400k, 14, 4_000, 400_000);
bench_rayon!(rayon_t16_n4k_m400k, 16, 4_000, 400_000);
bench_rayon!(rayon_t20_n4k_m400k, 20, 4_000, 400_000);
bench_rayon!(rayon_t24_n4k_m400k, 24, 4_000, 400_000);
bench_rayon!(rayon_t28_n4k_m400k, 28, 4_000, 400_000);
bench_rayon!(rayon_t32_n4k_m400k, 32, 4_000, 400_000);

// bench_rayon!(rayon_t1_n40k_m400k, 1, 40_000, 400_000);
// bench_rayon!(rayon_t4_n40k_m400k, 4, 40_000, 400_000);
// bench_rayon!(rayon_t8_n40k_m400k, 8, 40_000, 400_000);
// bench_rayon!(rayon_t16_n40k_m400k, 16, 40_000, 400_000);
// bench_rayon!(rayon_t32_n40k_m400k, 32, 40_000, 400_000);

// bench_rayon!(rayon_t1_n40k_m4m, 1, 40_000, 4_000_000);
// bench_rayon!(rayon_t4_n40k_m4m, 4, 40_000, 4_000_000);
// bench_rayon!(rayon_t8_n40k_m4m, 8, 40_000, 4_000_000);
// bench_rayon!(rayon_t16_n40k_m4m, 16, 40_000, 4_000_000);
// bench_rayon!(rayon_t32_n40k_m4m, 32, 40_000, 4_000_000);

bench_custom!(custom_t1_n4k_m400k, 1, 4_000, 400_000);
bench_custom!(custom_t2_n4k_m400k, 2, 4_000, 400_000);
bench_custom!(custom_t4_n4k_m400k, 4, 4_000, 400_000);
bench_custom!(custom_t6_n4k_m400k, 6, 4_000, 400_000);
bench_custom!(custom_t8_n4k_m400k, 8, 4_000, 400_000);
bench_custom!(custom_t10_n4k_m400k, 10, 4_000, 400_000);
bench_custom!(custom_t12_n4k_m400k, 12, 4_000, 400_000);
bench_custom!(custom_t14_n4k_m400k, 14, 4_000, 400_000);
bench_custom!(custom_t16_n4k_m400k, 16, 4_000, 400_000);
bench_custom!(custom_t20_n4k_m400k, 20, 4_000, 400_000);
bench_custom!(custom_t24_n4k_m400k, 24, 4_000, 400_000);
bench_custom!(custom_t28_n4k_m400k, 28, 4_000, 400_000);
bench_custom!(custom_t32_n4k_m400k, 32, 4_000, 400_000);
