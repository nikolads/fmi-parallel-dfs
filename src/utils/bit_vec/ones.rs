use std::slice;
use std::sync::atomic::Ordering;

use super::{AtomicB, BitSlice, B, B_BITS};

impl<'a> BitSlice<'a> {
    pub fn ones(self) -> Ones<'a> {
        match self.storage.len() {
            0 => Ones {
                first: 0,
                first_offset: 0,
                last: 0,
                rest: (&[]).iter(),
            },
            1 => {
                let mut head = self.storage.get(0).unwrap().load(Ordering::SeqCst);

                let mask = !((1 << self.start_offset) - 1);
                head &= mask;

                let mask_last_n = (B_BITS - (self.start_offset + self.nbits) % B_BITS) % B_BITS;
                let mask = if mask_last_n == 0 {
                    !0
                } else {
                    (1 << (B_BITS - mask_last_n)) - 1
                };
                head &= mask;

                Ones {
                    first: head,
                    first_offset: -(self.start_offset as isize),
                    last: 0,
                    rest: (&[]).iter(),
                }
            },
            len => {
                let (head, tail) = self.storage.split_first().unwrap();
                let (last, mid) = tail.split_last().unwrap();

                let head = {
                    let mask = !((1 << self.start_offset) - 1);
                    head.load(Ordering::SeqCst) & mask
                };

                let (mid, last) = if self.start_offset + self.nbits % B_BITS == 0 {
                    (tail, 0)
                } else {
                    let mask_last_n = (B_BITS - (self.start_offset + self.nbits) % B_BITS) % B_BITS;
                    let mask = if mask_last_n == 0 {
                        !0
                    } else {
                        (1 << (B_BITS - mask_last_n)) - 1
                    };

                    (mid, last.load(Ordering::SeqCst) & mask)
                };

                Ones {
                    first: head,
                    first_offset: -(self.start_offset as isize),
                    last: last,
                    rest: mid.iter(),
                }
            },
        }
    }
}

#[derive(Debug)]
pub struct Ones<'a> {
    first: B,
    first_offset: isize,
    last: B,
    rest: slice::Iter<'a, AtomicB>,
}

impl<'a> Iterator for Ones<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.first == 0 {
            match self.rest.next() {
                Some(w) => {
                    self.first = w.load(Ordering::SeqCst);
                    self.first_offset += B_BITS as isize;
                },
                None => {
                    if self.last == 0 {
                        return None;
                    } else {
                        self.first = self.last;
                        self.first_offset += B_BITS as isize;
                        self.last = 0;
                    }
                },
            }
        }

        // from the current block, isolate the
        // LSB and subtract 1, producing k:
        // a block with a number of set bits
        // equal to the index of the LSB
        let k = (self.first & (!self.first + 1)) - 1;
        // update block, removing the LSB
        self.first = self.first & (self.first - 1);
        // return offset + (index of LSB)
        Some((self.first_offset + (B::count_ones(k) as isize)) as usize)
    }
}

impl<'a> DoubleEndedIterator for Ones<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        while self.last == 0 {
            match self.rest.next_back() {
                Some(w) => {
                    self.last = w.load(Ordering::SeqCst);
                },
                None => {
                    if self.first == 0 {
                        return None;
                    } else {
                        self.last = self.first;
                        self.first = 0;
                        self.first_offset -= B_BITS as isize;
                    }
                },
            }
        }

        let i = B_BITS - 1 - self.last.leading_zeros() as usize;

        self.last = self.last & !(1 << i);

        Some((self.first_offset + (self.rest.as_slice().len() * B_BITS + B_BITS) as isize + i as isize) as usize)
    }
}
