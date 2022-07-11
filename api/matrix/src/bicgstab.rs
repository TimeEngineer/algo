use crate::SMatrix;
use vector::VectorOp;

#[cfg(test)]
const EPSILON: f64 = 1e-10;

fn add<const N: usize>(a: [f64; N], b: [f64; N]) -> [f64; N] {
    let mut out = [0.; N];
    for i in 0..N {
        out[i] = a[i] + b[i]
    }
    out
}
fn sub<const N: usize>(a: [f64; N], b: [f64; N]) -> [f64; N] {
    let mut out = [0.; N];
    for i in 0..N {
        out[i] = a[i] - b[i]
    }
    out
}
fn mul_with<const N: usize>(a: [f64; N], b: f64) -> [f64; N] {
    let mut out = [0.; N];
    for i in 0..N {
        out[i] = a[i] * b
    }
    out
}

impl<const N: usize> SMatrix<f64, N> {
    const EPSILON: f64 = 1e-10;

    pub fn bicgstab(&self, mut r: [f64; N]) -> [f64; N] {
        let mut x = [0.0; N];
        let rp = r;
        let mut p = r;

        loop {
            let s0 = VectorOp::dot(&r, &rp);
            let s1 = self * p;
            let a = s0 / VectorOp::dot(&s1, &rp);
            let s = sub(r, mul_with(s1, a));
            let s2 = self * s;
            let w = VectorOp::dot(&s2, &s) / VectorOp::dot(&s2, &s2);
            x = add(x, add(mul_with(p, a), mul_with(s, w)));
            r = sub(s, mul_with(s2, w));
            if VectorOp::dot(&r, &r) < Self::EPSILON {
                break;
            }
            let b = a / w * VectorOp::dot(&r, &rp) / s0;
            p = add(r, mul_with(sub(p, mul_with(s1, w)), b));
        }

        x
    }
}

#[test]
fn bicgstab() {
    const ROW0: [f64; 6] = [1., 0., 0., 0., 0., 0.];
    const ROW1: [f64; 6] = [0., 1., -1., 0., 0., 0.];
    const ROW2: [f64; 6] = [0., 0., 0., 0., -1., 1.];
    const ROW3: [f64; 6] = [0., 0., -1e-3, 3e-3, -1e-3, -1e-3];
    const ROW4: [f64; 6] = [-1e-3, 2e-3, 0., 0., 0., 0.];
    const ROW5: [f64; 6] = [0., 0., 0., -1e-3, 2e-3, 0.];
    const MATRIX: [[f64; 6]; 6] = [ROW0, ROW1, ROW2, ROW3, ROW4, ROW5];
    const B: [f64; 6] = [1.0, 2.0, 1.0, 0.0, 0.0, 0.0];

    let m = SMatrix::from_raw(MATRIX);

    let x = m.bicgstab(B);

    let b = &m * x;
    for i in 0..6 {
        assert!((B[i] - b[i]).abs() < EPSILON);
    }
}
