#![macro_use]

use std::fmt::Debug;

macro_rules! str_vec {
    ($($x:expr), *) => (vec![$($x.to_string(),)*])
}

pub fn assert_float_approx(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-5);
}

pub fn assert_unsort_eq<T: Debug + Ord>(mut a: Vec<T>, mut b: Vec<T>) {
    a.sort();
    b.sort();
    assert_eq!(a, b);
}
