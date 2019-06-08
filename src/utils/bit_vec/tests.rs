use super::*;

#[test]
fn test_bit_vec_remove() {
    let a = BitVec::new(1001);

    assert_eq!(a.swap(1, true), false);
    assert_eq!(a.swap(1, false), true);

    assert_eq!(a.swap(100, true), false);
    assert_eq!(a.swap(100, false), true);

    assert_eq!(a.swap(1000, true), false);
    assert_eq!(a.swap(1000, false), true);
}

#[test]
fn ones() {
    let a = BitVec::new(100);

    a.set(1, true);
    a.set(2, true);
    a.set(31, true);
    a.set(32, true);
    a.set(33, true);
    a.set(63, true);
    a.set(64, true);
    a.set(65, true);

    let mut iter = a.slice(..).ones();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(31));
    assert_eq!(iter.next(), Some(32));
    assert_eq!(iter.next(), Some(33));
    assert_eq!(iter.next(), Some(63));
    assert_eq!(iter.next(), Some(64));
    assert_eq!(iter.next(), Some(65));
    assert_eq!(iter.next(), None);
}

#[test]
fn ones_offset() {
    let a = BitVec::new(100);

    a.set(1, true);
    a.set(2, true);
    a.set(31, true);
    a.set(32, true);
    a.set(33, true);
    a.set(63, true);
    a.set(64, true);
    a.set(65, true);

    dbg!(&a);

    let mut iter = dbg!(a.slice(2..64).ones());
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(29));
    assert_eq!(iter.next(), Some(30));
    assert_eq!(iter.next(), Some(31));
    assert_eq!(iter.next(), Some(61));
    assert_eq!(iter.next(), None);
}

#[test]
fn ones_front_and_back() {
    let a = BitVec::new(100);

    a.set(1, true);
    a.set(2, true);
    a.set(11, true);
    a.set(12, true);
    a.set(29, true);
    a.set(30, true);
    a.set(31, true);

    dbg!(&a);

    let mut iter = dbg!(a.slice(2..30).ones());
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(9));
    assert_eq!(iter.next(), Some(10));
    assert_eq!(iter.next(), Some(27));
    assert_eq!(iter.next(), None);
}

#[test]
fn ones_rev() {
    let a = BitVec::new(100);

    a.set(1, true);
    a.set(2, true);
    a.set(31, true);
    a.set(32, true);
    a.set(33, true);
    a.set(63, true);
    a.set(64, true);
    a.set(65, true);

    let mut iter = a.slice(..).ones().rev();
    assert_eq!(iter.next(), Some(65));
    assert_eq!(iter.next(), Some(64));
    assert_eq!(iter.next(), Some(63));
    assert_eq!(iter.next(), Some(33));
    assert_eq!(iter.next(), Some(32));
    assert_eq!(iter.next(), Some(31));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
}

#[test]
fn ones_rev_offset() {
    let a = BitVec::new(100);

    a.set(1, true);
    a.set(2, true);
    a.set(31, true);
    a.set(32, true);
    a.set(33, true);
    a.set(63, true);
    a.set(64, true);
    a.set(65, true);

    dbg!(&a);

    let mut iter = dbg!(a.slice(2..64).ones().rev());
    assert_eq!(iter.next(), Some(61));
    assert_eq!(iter.next(), Some(31));
    assert_eq!(iter.next(), Some(30));
    assert_eq!(iter.next(), Some(29));
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), None);
}

#[test]
fn ones_rev_front_and_back() {
    let a = BitVec::new(100);

    a.set(1, true);
    a.set(2, true);
    a.set(11, true);
    a.set(12, true);
    a.set(29, true);
    a.set(30, true);
    a.set(31, true);

    dbg!(&a);

    let mut iter = dbg!(a.slice(2..30).ones().rev());
    assert_eq!(iter.next(), Some(27));
    assert_eq!(iter.next(), Some(10));
    assert_eq!(iter.next(), Some(9));
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), None);
}
