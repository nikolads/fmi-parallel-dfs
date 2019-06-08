use std::slice;
use std::sync::atomic::Ordering;

use super::{AtomicB, BitSlice, B, B_BITS};

impl<'a> BitSlice<'a> {
    pub fn ones(self) -> Ones<'a> {
        match self.storage.len() {
            0 => Ones {
                head: 0,
                head_offset: 0,
                tail: Blocks {
                    slice: (&[]).iter(),
                    last: None,
                    mask_last_n: 0,
                },
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
                    head,
                    head_offset: -(self.start_offset as isize),
                    tail: Blocks {
                        slice: (&[]).iter(),
                        last: None,
                        mask_last_n: 0,
                    },
                }
            },
            _ => {
                let (head, tail) = match self.storage.split_first() {
                    Some((head, tail)) => {
                        let mask = !((1 << self.start_offset) - 1);
                        (head.load(Ordering::SeqCst) & mask, tail)
                    },
                    None => (0, &[][..]),
                };

                let (mid, last) = if self.start_offset + self.nbits % B_BITS == 0 {
                    (tail, None)
                } else {
                    match tail.split_last() {
                        Some((last, mid)) => (mid, Some(last)),
                        None => (&[][..], None),
                    }
                };

                let tail_blocks = Blocks {
                    slice: mid.iter(),
                    last,
                    mask_last_n: (B_BITS - (self.start_offset + self.nbits) % B_BITS) % B_BITS,
                };

                Ones {
                    head,
                    head_offset: -(self.start_offset as isize),
                    tail: tail_blocks,
                }
            },
        }
    }
}

#[derive(Debug)]
struct Blocks<'a> {
    slice: slice::Iter<'a, AtomicB>,
    last: Option<&'a AtomicB>,
    mask_last_n: usize,
}

impl<'a> Iterator for Blocks<'a> {
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        self.slice
            .next()
            .map(|block| block.load(Ordering::SeqCst))
            .or_else(|| {
                self.last.take().map(|block| {
                    let mask = if self.mask_last_n == 0 {
                        !0
                    } else {
                        (1 << (B_BITS - self.mask_last_n)) - 1
                    };
                    block.load(Ordering::SeqCst) & mask
                })
            })
    }
}

#[derive(Debug)]
pub struct Ones<'a> {
    head: B,
    head_offset: isize,
    tail: Blocks<'a>,
}

impl<'a> Iterator for Ones<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.head == 0 {
            match self.tail.next() {
                Some(w) => {
                    self.head = w;
                    self.head_offset += B_BITS as isize;
                },
                None => return None,
            }
        }

        // from the current block, isolate the
        // LSB and subtract 1, producing k:
        // a block with a number of set bits
        // equal to the index of the LSB
        let k = (self.head & (!self.head + 1)) - 1;
        // update block, removing the LSB
        self.head = self.head & (self.head - 1);
        // return offset + (index of LSB)
        Some((self.head_offset + (B::count_ones(k) as isize)) as usize)
    }
}

impl<'a> DoubleEndedIterator for Ones<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // TODO
        self.next()
    }
}
