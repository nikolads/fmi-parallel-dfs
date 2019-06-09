#![feature(decl_macro)]
#![feature(test)]

extern crate parallel_dfs;
extern crate rand;
extern crate rayon;
extern crate test;

use parallel_dfs::graph::AdjLists;
use parallel_dfs::graph::adj_lists::mirror;
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

        let array: Vec<Vec<usize>> = {
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
            bencher.iter(|| {
                let mut array = array.clone();
                mirror::$fn(&mut array)
            });
        });
    }
}

bench!(seq_n4k_m400k, seq, 1, 4_000, 400_000);

bench!(mutex_n4k_m400k_t01, mutex, 1, 4_000, 400_000);
bench!(mutex_n4k_m400k_t02, mutex, 2, 4_000, 400_000);
bench!(mutex_n4k_m400k_t04, mutex, 4, 4_000, 400_000);
bench!(mutex_n4k_m400k_t06, mutex, 6, 4_000, 400_000);
bench!(mutex_n4k_m400k_t08, mutex, 8, 4_000, 400_000);
bench!(mutex_n4k_m400k_t10, mutex, 10, 4_000, 400_000);
bench!(mutex_n4k_m400k_t12, mutex, 12, 4_000, 400_000);
bench!(mutex_n4k_m400k_t14, mutex, 14, 4_000, 400_000);
bench!(mutex_n4k_m400k_t16, mutex, 16, 4_000, 400_000);
bench!(mutex_n4k_m400k_t20, mutex, 20, 4_000, 400_000);
bench!(mutex_n4k_m400k_t24, mutex, 24, 4_000, 400_000);
bench!(mutex_n4k_m400k_t28, mutex, 28, 4_000, 400_000);
bench!(mutex_n4k_m400k_t32, mutex, 32, 4_000, 400_000);

bench!(spin_lock_n4k_m400k_t01, spin_lock, 1, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t02, spin_lock, 2, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t04, spin_lock, 4, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t06, spin_lock, 6, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t08, spin_lock, 8, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t10, spin_lock, 10, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t12, spin_lock, 12, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t14, spin_lock, 14, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t16, spin_lock, 16, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t20, spin_lock, 20, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t24, spin_lock, 24, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t28, spin_lock, 28, 4_000, 400_000);
bench!(spin_lock_n4k_m400k_t32, spin_lock, 32, 4_000, 400_000);

bench!(queue_n4k_m400k_t01, queue, 1, 4_000, 400_000);
bench!(queue_n4k_m400k_t02, queue, 2, 4_000, 400_000);
bench!(queue_n4k_m400k_t04, queue, 4, 4_000, 400_000);
bench!(queue_n4k_m400k_t06, queue, 6, 4_000, 400_000);
bench!(queue_n4k_m400k_t08, queue, 8, 4_000, 400_000);
bench!(queue_n4k_m400k_t10, queue, 10, 4_000, 400_000);
bench!(queue_n4k_m400k_t12, queue, 12, 4_000, 400_000);
bench!(queue_n4k_m400k_t14, queue, 14, 4_000, 400_000);
bench!(queue_n4k_m400k_t16, queue, 16, 4_000, 400_000);
bench!(queue_n4k_m400k_t20, queue, 20, 4_000, 400_000);
bench!(queue_n4k_m400k_t24, queue, 24, 4_000, 400_000);
bench!(queue_n4k_m400k_t28, queue, 28, 4_000, 400_000);
bench!(queue_n4k_m400k_t32, queue, 32, 4_000, 400_000);

