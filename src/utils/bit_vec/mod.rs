use std::fmt::Debug;
use std::ops::{Bound, RangeBounds};
use std::sync::atomic::Ordering;

mod iter;
mod ones;

pub use self::iter::Iter;
pub use self::ones::Ones;

type AtomicB = std::sync::atomic::AtomicU64;
type B = u64;
static B_BITS: usize = std::mem::size_of::<B>() * 8;

#[derive(Debug)]
pub struct BitVec {
    storage: Box<[AtomicB]>,
    nbits: usize,
}

impl BitVec {
    pub fn new(nbits: usize) -> Self {
        let blocks = if nbits % B_BITS == 0 {
            nbits / B_BITS
        } else {
            nbits / B_BITS + 1
        };

        let mut storage = Vec::with_capacity(blocks);
        storage.resize_with(blocks, AtomicB::default);

        BitVec {
            storage: storage.into_boxed_slice(),
            nbits,
        }
    }

    pub fn get(&self, index: usize) -> Option<bool> {
        self.as_full_slice().get(index)
    }

    pub fn set(&self, index: usize, val: bool) {
        self.as_full_slice().set(index, val)
    }

    pub fn swap(&self, index: usize, val: bool) -> bool {
        self.as_full_slice().swap(index, val)
    }

    /// Returns the total number of bits in this vector
    #[inline]
    pub fn len(&self) -> usize {
        self.nbits
    }

    /// Returns true if there are no bits in this vector
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn slice<R>(&self, index: R) -> BitSlice
    where
        R: RangeBounds<usize> + Debug,
    {
        self.as_full_slice().slice(index)
    }

    /// Turn this BitVec into a BitSlice
    ///
    /// Used for delegating method implementation to the slice methods.
    /// Hopefully the conversion is compiled out in release builds.
    #[inline]
    fn as_full_slice(&self) -> BitSlice {
        BitSlice {
            storage: &self.storage,
            start_offset: 0,
            nbits: self.nbits,
        }
    }

    pub fn iter(&self) -> Iter {
        self.as_full_slice().into_iter()
    }
}

#[derive(Debug, Clone)]
pub struct BitSlice<'a> {
    storage: &'a [AtomicB],
    start_offset: usize,
    nbits: usize,
}

impl<'a> BitSlice<'a> {
    pub fn get(&self, index: usize) -> Option<bool> {
        if index >= self.nbits {
            return None;
        }

        let index = index + self.start_offset;

        let w = index / B_BITS;
        let b = index % B_BITS;
        self.storage
            .get(w)
            .map(|block| (block.load(Ordering::Acquire) & (1 << b)) != 0)
    }

    pub fn set(&self, index: usize, val: bool) {
        assert!(
            index < self.nbits,
            "index out of bounds: index is {:?} but len is {:?}",
            index,
            self.nbits
        );

        let index = index + self.start_offset;

        let w = index / B_BITS;
        let b = index % B_BITS;
        let flag = 1 << b;

        match val {
            true => self.storage[w].fetch_or(flag, Ordering::Release),
            false => self.storage[w].fetch_and(!flag, Ordering::Release),
        };
    }

    pub fn swap(&self, index: usize, val: bool) -> bool {
        assert!(
            index < self.nbits,
            "index out of bounds: index is {:?} but len is {:?}",
            index,
            self.nbits
        );

        let index = index + self.start_offset;

        let w = index / B_BITS;
        let b = index % B_BITS;
        let flag = 1 << b;

        let old = match val {
            true => self.storage[w].fetch_or(flag, Ordering::AcqRel),
            false => self.storage[w].fetch_and(!flag, Ordering::AcqRel),
        };

        old & flag != 0
    }

    /// Returns the total number of bits in this vector
    #[inline]
    pub fn len(&self) -> usize {
        self.nbits
    }

    /// Returns true if there are no bits in this vector
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn slice<R>(&self, index: R) -> BitSlice<'a>
    where
        R: RangeBounds<usize> + Debug,
    {
        let input_index_start = match index.start_bound() {
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x.checked_add(1).unwrap(),
            Bound::Unbounded => 0,
        };

        let input_index_end = match index.end_bound() {
            Bound::Included(&x) => x.checked_add(1).unwrap(),
            Bound::Excluded(&x) => x,
            Bound::Unbounded => self.nbits,
        };

        let input_index = input_index_start..input_index_end;

        assert!(
            input_index.end <= self.nbits,
            "index out of bounds: index is {:?} but len is {:?}",
            index,
            self.nbits,
        );

        assert!(
            input_index.start <= input_index.end,
            "start of index range is greater than the end: index is {:?}",
            index,
        );

        let actual_index =
            (input_index.start + self.start_offset)..(input_index.end + self.start_offset);

        let start_bucket = actual_index.start / B_BITS;
        let end_bucket = if actual_index.end % B_BITS == 0 {
            actual_index.end / B_BITS
        } else {
            actual_index.end / B_BITS + 1
        };

        BitSlice {
            storage: &self.storage[start_bucket..end_bucket],
            start_offset: actual_index.start % B_BITS,
            nbits: actual_index.len(),
        }
    }
}

#[cfg(test)]
mod tests;
