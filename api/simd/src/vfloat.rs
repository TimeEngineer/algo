use crate::float::F32x4;
use rand::rngs::ThreadRng;

#[cfg(test)]
const EPSILON: f32 = 1e-6;

// ----------------------------------------
// VF32x4
// ----------------------------------------

pub struct VF32x4;

impl VF32x4 {
    pub fn alloc(n: usize) -> Vec<F32x4> {
        vec![F32x4::zero(); n]
    }

    pub fn rand(n: usize, rng: &mut ThreadRng) -> Vec<F32x4> {
        let mut v = Self::alloc(n);
        for a in v.iter_mut() {
            *a = F32x4::rand(rng);
        }
        v
    }

    pub fn randomize(v: &mut [F32x4], rng: &mut ThreadRng) {
        for a in v.iter_mut() {
            *a = F32x4::rand(rng);
        }
    }

    pub fn broadcast_with(v: &mut [F32x4], b: F32x4) {
        for a in v.iter_mut() {
            *a = b;
        }
    }

    pub fn into_vf(v: Vec<F32x4>) -> Vec<f32> {
        let mut v = core::mem::ManuallyDrop::new(v);
        let ptr = v.as_mut_ptr() as _;
        let len = v.len() * 4;
        let cap = v.capacity() * 4;
        unsafe { Vec::from_raw_parts(ptr, len, cap) }
    }

    pub fn add_assign(v0: &mut [F32x4], v1: &[F32x4]) {
        for (a, &b) in v0.iter_mut().zip(v1.iter()) {
            *a += b;
        }
    }

    pub fn add_assign_with(v: &mut [F32x4], b: F32x4) {
        for a in v.iter_mut() {
            *a += b;
        }
    }

    pub fn sub_assign(v0: &mut [F32x4], v1: &[F32x4]) {
        for (a, &b) in v0.iter_mut().zip(v1.iter()) {
            *a -= b;
        }
    }

    pub fn sub_assign_with(v: &mut [F32x4], b: F32x4) {
        for a in v.iter_mut() {
            *a -= b;
        }
    }

    pub fn mul_assign(v0: &mut [F32x4], v1: &[F32x4]) {
        for (a, &b) in v0.iter_mut().zip(v1.iter()) {
            *a *= b;
        }
    }

    pub fn mul_assign_with(v: &mut [F32x4], b: F32x4) {
        for a in v.iter_mut() {
            *a *= b;
        }
    }

    pub fn div_assign(v0: &mut [F32x4], v1: &[F32x4]) {
        for (a, &b) in v0.iter_mut().zip(v1.iter()) {
            *a /= b;
        }
    }

    pub fn div_assign_with(v: &mut [F32x4], b: F32x4) {
        for a in v.iter_mut() {
            *a /= b;
        }
    }

    pub fn dot(v0: &[F32x4], v1: &[F32x4]) -> F32x4 {
        let mut c = F32x4::zero();
        for (&a, &b) in v0.iter().zip(v1.iter()) {
            c += a * b;
        }
        c.sum()
    }

    pub fn sum(v: &[F32x4]) -> F32x4 {
        let mut c = F32x4::zero();
        for &a in v.iter() {
            c += a;
        }
        c.sum()
    }

    pub fn rev_half(v: &mut [F32x4]) {
        for a in v.iter_mut() {
            *a = a.rev_half();
        }
    }

    pub fn rev(v: &mut [F32x4]) {
        for a in v.iter_mut() {
            *a = a.rev();
        }
    }

    pub fn rnorm(v: &[F32x4]) -> F32x4 {
        let mut n = F32x4::zero();
        for &a in v.iter() {
            n += a * a;
        }
        n.sum().rsqrt()
    }

    pub fn normalize(v: &mut [F32x4]) {
        let n = Self::rnorm(v);
        for a in v.iter_mut() {
            *a *= n;
        }
    }
}

#[test]
fn rand() {
    let mut rng = rand::thread_rng();
    let _ = VF32x4::rand(2, &mut rng);
}

#[test]
fn new() {
    let mut v = VF32x4::alloc(2);
    v[0] = F32x4::new(0., 1., 2., 3.);
    v[1] = F32x4::new(4., 5., 6., 7.);

    assert_eq!(&VF32x4::into_vf(v), &[0., 1., 2., 3., 4., 5., 6., 7.]);
}

#[test]
fn add_assign() {
    let mut v0 = VF32x4::alloc(2);
    v0[0] = F32x4::new(0., 1., 2., 3.);
    v0[1] = F32x4::new(4., 5., 6., 7.);

    let mut v1 = VF32x4::alloc(2);
    v1[0] = F32x4::new(0., 1., 2., 3.);
    v1[1] = F32x4::new(4., 5., 6., 7.);

    VF32x4::add_assign(&mut v0, &v1);
    assert_eq!(&VF32x4::into_vf(v0), &[0., 2., 4., 6., 8., 10., 12., 14.]);
}

#[test]
fn sub_assign() {
    let mut v0 = VF32x4::alloc(2);
    v0[0] = F32x4::new(0., 1., 2., 3.);
    v0[1] = F32x4::new(4., 5., 6., 7.);

    let mut v1 = VF32x4::alloc(2);
    v1[0] = F32x4::new(0., 1., 2., 3.);
    v1[1] = F32x4::new(4., 5., 6., 7.);

    VF32x4::sub_assign(&mut v0, &v1);
    assert_eq!(&VF32x4::into_vf(v0), &[0., 0., 0., 0., 0., 0., 0., 0.]);
}

#[test]
fn mul_assign() {
    let mut v0 = VF32x4::alloc(2);
    v0[0] = F32x4::new(0., 1., 2., 3.);
    v0[1] = F32x4::new(4., 5., 6., 7.);

    let mut v1 = VF32x4::alloc(2);
    v1[0] = F32x4::new(0., 1., 2., 3.);
    v1[1] = F32x4::new(4., 5., 6., 7.);

    VF32x4::mul_assign(&mut v0, &v1);
    assert_eq!(&VF32x4::into_vf(v0), &[0., 1., 4., 9., 16., 25., 36., 49.]);
}

#[test]
fn div_assign() {
    let mut v0 = VF32x4::alloc(2);
    v0[0] = F32x4::new(0., 1., 2., 3.);
    v0[1] = F32x4::new(4., 5., 6., 7.);

    let mut v1 = VF32x4::alloc(2);
    v1[0] = F32x4::new(1., 1., 2., 3.);
    v1[1] = F32x4::new(4., 5., 6., 7.);

    VF32x4::div_assign(&mut v0, &v1);
    assert_eq!(&VF32x4::into_vf(v0), &[0., 1., 1., 1., 1., 1., 1., 1.]);
}

#[test]
fn dot() {
    let mut v0 = VF32x4::alloc(2);
    v0[0] = F32x4::new(0., 1., 2., 3.);
    v0[1] = F32x4::new(4., 5., 6., 7.);

    let mut v1 = VF32x4::alloc(2);
    v1[0] = F32x4::new(0., 1., 2., 3.);
    v1[1] = F32x4::new(4., 5., 6., 7.);

    let v = VF32x4::dot(&v0, &v1);
    let n = 7.;
    let sum = (n * (n + 1.) * (2. * n + 1.)) / 6.;

    assert_eq!(v.to_array(), [sum, sum, sum, sum]);
}

#[test]
fn sum() {
    let mut v = VF32x4::alloc(2);
    v[0] = F32x4::new(0., 1., 2., 3.);
    v[1] = F32x4::new(4., 5., 6., 7.);

    let a = VF32x4::sum(&v);
    assert_eq!(a.to_array(), [28., 28., 28., 28.]);
}

#[test]
fn rev() {
    let mut v0 = VF32x4::alloc(2);
    v0[0] = F32x4::new(1., 2., 4., 8.);
    v0[1] = F32x4::new(1., 0.5, 0.25, 0.125);

    VF32x4::rev(&mut v0);

    let mut v1 = VF32x4::alloc(2);
    v1[0] = F32x4::new(1., 0.5, 0.25, 0.125);
    v1[1] = F32x4::new(1., 2., 4., 8.);

    VF32x4::sub_assign(&mut v0, &v1);

    for x in VF32x4::into_vf(v0).iter() {
        assert!(x.abs() < EPSILON);
    }
}

#[test]
fn rnorm() {
    let mut v0 = VF32x4::alloc(2);
    v0[0] = F32x4::new(0., 1., 2., 3.);
    v0[1] = F32x4::new(4., 5., 6., 7.);

    let n = VF32x4::rnorm(&v0);

    let sum: f32 = 140.;
    let rnorm = 1. / sum.sqrt();
    let b = F32x4::broadcast_with(rnorm);

    for x in (b - n).to_array().iter() {
        assert!(x.abs() < EPSILON);
    }
}

#[test]
fn normalize() {
    let mut v0 = VF32x4::alloc(2);
    v0[0] = F32x4::new(0., 1., 2., 3.);
    v0[1] = F32x4::new(4., 5., 6., 7.);

    VF32x4::normalize(&mut v0);

    let mut v1 = VF32x4::alloc(2);
    let sum: f32 = 140.;
    let norm = sum.sqrt();
    v1[0] = F32x4::new(0., 1. / norm, 2. / norm, 3. / norm);
    v1[1] = F32x4::new(4. / norm, 5. / norm, 6. / norm, 7. / norm);

    VF32x4::sub_assign(&mut v0, &v1);

    for x in VF32x4::into_vf(v0).iter() {
        assert!(x.abs() < EPSILON);
    }
}
