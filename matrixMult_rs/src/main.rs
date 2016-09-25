extern crate ndarray;
extern crate rand;
extern crate ndarray_rand;
extern crate time;
extern crate rayon;

use ndarray::{Axis, ArrayView, ArrayViewMut, Ix};
use rayon::prelude::*;
use ndarray::OwnedArray;
use ndarray_rand::RandomExt;
use rand::distributions::Range;
use std::num::*;

// various array views for divide-and-conquering
pub type MatView<'a, A> = ArrayView<'a, A, (Ix, Ix)>;
pub type MatViewMut<'a, A> = ArrayViewMut<'a, A, (Ix, Ix)>;

pub fn matrix_dot_safe(left: &MatView<f64>, right: &MatView<f64>, init: &mut MatViewMut<f64>) {
    let res = left.dot(right);
    init.zip_mut_with(&res, |x, y| *x = *y)
}


pub const BLOCKSIZE: usize = 100;

// parallelized matrix multiplication via rayon.
pub fn matrix_dot_rayon(left: &MatView<f64>, right: &MatView<f64>, init: &mut MatViewMut<f64>) {

    let (m, k1) = left.dim();
    let (k2, n) = right.dim();
    assert_eq!(k1, k2);

    if m <= BLOCKSIZE && n <= BLOCKSIZE {
        matrix_dot_safe(left, right, init);
        return;
    } else if m > BLOCKSIZE {
        let mid = m / 2;
        let (left_0, left_1) = left.split_at(Axis(0), mid);
        let (mut init_left, mut init_right) = init.view_mut().split_at(Axis(0), mid);
        rayon::join(|| matrix_dot_rayon(&left_0, right, &mut init_left),
                    || matrix_dot_rayon(&left_1, right, &mut init_right));

    } else if n > BLOCKSIZE {
        let mid = n / 2;
        let (right_0, right_1) = right.split_at(Axis(1), mid);
        let (mut init_left, mut init_right) = init.view_mut().split_at(Axis(1), mid);
        rayon::join(|| matrix_dot_rayon(left, &right_0, &mut init_left),
                    || matrix_dot_rayon(left, &right_1, &mut init_right));
    }
}

fn main () {
    let mut sz = 2;
    for i in 0..10 {
    sz = sz*2;
    let mut c0 = OwnedArray::zeros((sz, sz));
    let mut c = OwnedArray::zeros((sz, sz));

    let x = OwnedArray::random((sz, sz), Range::new(0., 10.));
    let y = OwnedArray::random((sz, sz), Range::new(0., 10.));

    let seq_start = time::precise_time_ns();
    matrix_dot_safe(&x.view(), &y.view(), &mut c0.view_mut());
    let seq_time = time::precise_time_ns() - seq_start;
    let par_start = time::precise_time_ns();
    matrix_dot_rayon(&x.view(), &y.view(), &mut c.view_mut());
    let par_time = time::precise_time_ns() - par_start;
    println!("Matrix Size : {}", sz);
    println!("Sequential time  : {} ns", seq_time);
    println!("Parallel time    : {} ns", par_time);
    }

}
