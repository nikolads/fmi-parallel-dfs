use crossbeam::sync::SegQueue;
use rayon::prelude::*;
use spin::Mutex as SpinLock;

use std::sync::Mutex;

pub fn seq(lists: &mut Vec<Vec<usize>>) {
    for u in 0..lists.len() {
        let (before, after) = lists.split_at_mut(u);
        let (list, _) = after.split_first_mut().unwrap();

        for &v in list.iter() {
            match v {
                _ if v < u => before[v].push(u),
                _ => (),
            }
        }
    }
}

pub fn mutex(lists: &mut Vec<Vec<usize>>) {
    let locked = lists.iter_mut().map(|list| Mutex::new(list)).collect::<Vec<_>>();

    locked
        .par_iter()
        .enumerate()
        .for_each(|(u, list)| {
            // can't deadlock because v < u
            for &v in list.lock().unwrap().iter() {
                if v < u {
                    locked[v].lock().unwrap().push(u);
                }
            }
        })
}

pub fn spin_lock(lists: &mut Vec<Vec<usize>>) {
    let locked = lists.iter_mut().map(|list| SpinLock::new(list)).collect::<Vec<_>>();

    locked
        .par_iter()
        .enumerate()
        .for_each(|(u, list)| {
            // can't deadlock because v < u
            for &v in list.lock().iter() {
                if v < u {
                    locked[v].lock().push(u);
                }
            }
        })
}

pub fn queue(lists: &mut Vec<Vec<usize>>) {
    let queues = lists.iter().map(|_| SegQueue::new()).collect::<Vec<_>>();

    lists
        .par_iter()
        .enumerate()
        .for_each(|(u, list)| {
            for &v in list.iter() {
                queues[v].push(u);
            }
        });

    lists
        .par_iter_mut()
        .zip(queues)
        .for_each(|(list, queue)| {
            while let Some(v) = queue.try_pop() {
                list.push(v);
            }
        });
}
