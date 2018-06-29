#![feature(decl_macro)]
#![feature(test)]

extern crate parallel_dfs;
extern crate rand;
extern crate rayon;
extern crate test;

use parallel_dfs::graph::AdjLists;
use parallel_dfs::mirror;
use rand::distributions::Standard;
use rand::prelude::*;
use rand::prng::XorShiftRng;
use rayon::ThreadPoolBuilder;
use test::Bencher;

const SEED: [u8; 16] = [130, 241, 105, 144, 39, 87, 188, 11, 85, 171, 153, 10, 140, 0, 21, 127];

macro bench($name:ident, $fn:ident, $t:expr, $n:expr, $m:expr) {
    #[bench]
    fn $name(bencher: &mut Bencher) {
            let mut rng = XorShiftRng::from_seed(SEED);
            let graph = AdjLists::gen_directed($n, $m, rng.sample_iter(&Standard));

    let mut array: Vec<Vec<usize>> = {
            graph
                .vertices()
                .map(|v| graph.neighbours(v).filter(|&u| u < v).collect::<Vec<usize>>())
                .collect::<Vec<Vec<usize>>>()
        };

        let thread_pool = ThreadPoolBuilder::new()
            .num_threads($t)
            .build()
            .unwrap();

        thread_pool.install(|| {
            bencher.iter(|| mirror::$fn(&mut array));
        });
    }
}

bench!(seq_n4k_m400k, seq, 1, 4_000, 400_000);
bench!(mutex_n4k_m400k_t1, mutex, 1, 4_000, 400_000);
bench!(mutex_n4k_m400k_t2, mutex, 2, 4_000, 400_000);
bench!(mutex_n4k_m400k_t4, mutex, 4, 4_000, 400_000);
bench!(mutex_n4k_m400k_t6, mutex, 6, 4_000, 400_000);
bench!(mutex_n4k_m400k_t8, mutex, 8, 4_000, 400_000);
bench!(mutex_n4k_m400k_t10, mutex, 10, 4_000, 400_000);
bench!(mutex_n4k_m400k_t12, mutex, 12, 4_000, 400_000);
bench!(mutex_n4k_m400k_t14, mutex, 14, 4_000, 400_000);
bench!(mutex_n4k_m400k_t16, mutex, 16, 4_000, 400_000);
bench!(mutex_n4k_m400k_t20, mutex, 20, 4_000, 400_000);
bench!(mutex_n4k_m400k_t24, mutex, 24, 4_000, 400_000);
bench!(mutex_n4k_m400k_t28, mutex, 28, 4_000, 400_000);
bench!(mutex_n4k_m400k_t32, mutex, 32, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t1, spin_lock, 1, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t2, spin_lock, 2, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t4, spin_lock, 4, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t6, spin_lock, 6, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t8, spin_lock, 8, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t10, spin_lock, 10, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t12, spin_lock, 12, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t14, spin_lock, 14, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t16, spin_lock, 16, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t20, spin_lock, 20, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t24, spin_lock, 24, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t28, spin_lock, 28, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t32, spin_lock, 32, 4_000, 400_000);

bench!(seq_n40k_m400k, seq, 1, 40_000, 400_000);
bench!(mutex_n40k_m400k_t1, mutex, 1, 40_000, 400_000);
bench!(mutex_n40k_m400k_t4, mutex, 4, 40_000, 400_000);
bench!(mutex_n40k_m400k_t8, mutex, 8, 40_000, 400_000);
bench!(mutex_n40k_m400k_t16, mutex, 16, 40_000, 400_000);
bench!(mutex_n40k_m400k_t32, mutex, 32, 40_000, 400_000);
bench!(spin_lock_n40k_m400k_t1, spin_lock, 1, 40_000, 400_000);
bench!(spin_lock_n40k_m400k_t4, spin_lock, 4, 40_000, 400_000);
bench!(spin_lock_n40k_m400k_t8, spin_lock, 8, 40_000, 400_000);
bench!(spin_lock_n40k_m400k_t16, spin_lock, 16, 40_000, 400_000);
bench!(spin_lock_n40k_m400k_t32, spin_lock, 32, 40_000, 400_000);

bench!(seq_n4k_m4m, seq, 1, 4_000, 4_000_000);
bench!(mutex_n4k_m4m_t1, mutex, 1, 4_000, 4_000_000);
bench!(mutex_n4k_m4m_t4, mutex, 4, 4_000, 4_000_000);
bench!(mutex_n4k_m4m_t8, mutex, 8, 4_000, 4_000_000);
bench!(mutex_n4k_m4m_t16, mutex, 16, 4_000, 4_000_000);
bench!(mutex_n4k_m4m_t32, mutex, 32, 4_000, 4_000_000);
bench!(spin_lock_n4k_m4m_t1, spin_lock, 1, 4_000, 4_000_000);
bench!(spin_lock_n4k_m4m_t4, spin_lock, 4, 4_000, 4_000_000);
bench!(spin_lock_n4k_m4m_t8, spin_lock, 8, 4_000, 4_000_000);
bench!(spin_lock_n4k_m4m_t16, spin_lock, 16, 4_000, 4_000_000);
bench!(spin_lock_n4k_m4m_t32, spin_lock, 32, 4_000, 4_000_000);
