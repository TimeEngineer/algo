use core::ops::{Add, Mul, Sub};

pub struct VectorOp;

impl VectorOp {
    pub fn sum<T: Copy + Default + Add<Output = T>>(a: &[T]) -> T {
        let mut out = T::default();
        for ai in a.iter() {
            out = out + *ai;
        }
        out
    }

    pub fn dot<T: Copy + Default + Add<Output = T> + Mul<Output = T>>(a: &[T], b: &[T]) -> T {
        let mut out = T::default();
        for (ai, bi) in a.iter().zip(b) {
            out = out + *ai * *bi;
        }
        out
    }

    pub fn add_assign<T: Copy + Add<Output = T>>(a: &mut [T], b: &[T]) {
        for (ai, bi) in a.iter_mut().zip(b) {
            *ai = *ai + *bi;
        }
    }

    pub fn add_assign_with<T: Copy + Add<Output = T>>(a: &mut [T], x: T) {
        for ai in a.iter_mut() {
            *ai = *ai + x;
        }
    }

    pub fn sub<T: Copy + Sub<Output = T> + Default, const N: usize>(
        a: [T; N],
        b: [T; N],
    ) -> [T; N] {
        let mut out = [T::default(); N];
        for (i, (ai, bi)) in a.iter().zip(b).enumerate() {
            out[i] = *ai - bi;
        }
        out
    }

    pub fn sub_assign<T: Copy + Sub<Output = T>>(a: &mut [T], b: &[T]) {
        for (ai, bi) in a.iter_mut().zip(b) {
            *ai = *ai - *bi;
        }
    }

    pub fn sub_assign_with<T: Copy + Sub<Output = T>>(a: &mut [T], x: T) {
        for ai in a.iter_mut() {
            *ai = *ai - x;
        }
    }

    pub fn mul_assign<T: Copy + Mul<Output = T>>(a: &mut [T], b: &[T]) {
        for (ai, bi) in a.iter_mut().zip(b) {
            *ai = *ai * *bi;
        }
    }

    pub fn mul_assign_with<T: Copy + Mul<Output = T>>(a: &mut [T], x: T) {
        for ai in a.iter_mut() {
            *ai = *ai * x;
        }
    }
}

#[test]
fn sum() {
    let a = [1., 2., 3., 4.];

    assert_eq!(VectorOp::sum(&a), 10.);
}

#[test]
fn dot() {
    let a = [1., 2., 3., 4.];
    let b = [1., 2., 3., 4.];

    assert_eq!(VectorOp::dot(&a, &b), 30.);
}

#[test]
fn add_assign() {
    let mut a = [1., 2., 3., 4.];
    let b = [1., 2., 3., 4.];

    VectorOp::add_assign(&mut a, &b);
    assert_eq!(a, [2., 4., 6., 8.]);
}

#[test]
fn add_assign_with() {
    let mut a = [1., 2., 3., 4.];
    let x = 1.;

    VectorOp::add_assign_with(&mut a, x);
    assert_eq!(a, [2., 3., 4., 5.]);
}

#[test]
fn sub_assign() {
    let mut a = [1., 2., 3., 4.];
    let b = [1., 2., 3., 4.];

    VectorOp::sub_assign(&mut a, &b);
    assert_eq!(a, [0., 0., 0., 0.,]);
}

#[test]
fn sub_assign_with() {
    let mut a = [1., 2., 3., 4.];
    let x = 1.;

    VectorOp::sub_assign_with(&mut a, x);
    assert_eq!(a, [0., 1., 2., 3.]);
}

#[test]
fn mul_assign() {
    let mut a = [1., 2., 3., 4.];
    let b = [1., 2., 3., 4.];

    VectorOp::mul_assign(&mut a, &b);
    assert_eq!(a, [1., 4., 9., 16.]);
}

#[test]
fn mul_assign_with() {
    let mut a = [1., 2., 3., 4.];
    let x = 1.;

    VectorOp::mul_assign_with(&mut a, x);
    assert_eq!(a, [1., 2., 3., 4.]);
}
