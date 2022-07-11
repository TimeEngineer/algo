use crate::SMatrix;
use rand::Rng;
use vector::VectorOp;

fn rnorm(a: &[f64]) -> f64 {
    let mut out = 0.;
    for ai in a.iter() {
        out += *ai * *ai;
    }
    out.sqrt().recip()
}

fn normalize(a: &mut [f64]) {
    let rnorm = rnorm(a);
    for ai in a.iter_mut() {
        *ai *= rnorm;
    }
}

impl<const N: usize> SMatrix<f64, N> {
    pub fn spectral_layout<const M: usize>(&self, eps: f64) -> [[f64; N]; M] {
        let mut u = [[0.; N]; M];
        let mut d = [0.; N];
        let mut buffer = [0.; N];
        let mut rng = rand::thread_rng();

        // Initialize d
        for (di, row) in d.iter_mut().zip(&self.0) {
            *di = VectorOp::sum(row);
        }

        // 1 / tr(D)
        let rtrd = VectorOp::sum(&d).recip();

        // --------------------
        // Compute u
        // --------------------
        for k in 0..M {
            let mut step = 0.;
            rng.fill(&mut u[k][..]);
            normalize(&mut u[k][..]);

            loop {
                // D-orthogonalize against previous eigenvectors
                let a = VectorOp::dot(&u[k], &d) * rtrd;
                VectorOp::sub_assign_with(&mut u[k], a);

                for l in 0..k {
                    buffer.copy_from_slice(&d);
                    VectorOp::mul_assign(&mut buffer, &u[l]);
                    let a = VectorOp::dot(&u[k], &buffer) * VectorOp::dot(&u[l], &buffer).recip();
                    buffer.copy_from_slice(&u[l]);
                    VectorOp::mul_assign_with(&mut buffer, a);
                    VectorOp::sub_assign(&mut u[k], &buffer);
                }

                // Multiply with 0.5 (I + D^-1 A)
                for i in 0..N {
                    let a = VectorOp::dot(&self[i], &u[k]) / d[i];
                    buffer[i] = 0.5 * (u[k][i] + a);
                }

                // Normalization
                normalize(&mut buffer);

                let last_step = step;
                step = VectorOp::dot(&buffer, &u[k]);

                if step - last_step < eps {
                    break;
                }

                u[k].copy_from_slice(&buffer);
            }
            u[k].copy_from_slice(&buffer);
        }
        u
    }
}

#[test]
fn spectral_layout2() {
    const ROW0: [f64; 5] = [0., 1., 1., 1., 1.];
    const ROW1: [f64; 5] = [1., 0., 1., 1., 1.];
    const ROW2: [f64; 5] = [1., 1., 0., 1., 1.];
    const ROW3: [f64; 5] = [1., 1., 1., 0., 1.];
    const ROW4: [f64; 5] = [1., 1., 1., 1., 0.];
    const MATRIX: [[f64; 5]; 5] = [ROW0, ROW1, ROW2, ROW3, ROW4];

    let graph = SMatrix::from_raw(MATRIX);
    let out = graph.spectral_layout::<2>(1e-8);

    assert!(VectorOp::dot(&out[0], &out[1]).abs() < 1e-8);
}

#[test]
fn spectral_layout3() {
    const ROW0: [f64; 5] = [0., 1., 1., 1., 1.];
    const ROW1: [f64; 5] = [1., 0., 1., 1., 1.];
    const ROW2: [f64; 5] = [1., 1., 0., 1., 1.];
    const ROW3: [f64; 5] = [1., 1., 1., 0., 1.];
    const ROW4: [f64; 5] = [1., 1., 1., 1., 0.];
    const MATRIX: [[f64; 5]; 5] = [ROW0, ROW1, ROW2, ROW3, ROW4];

    let graph = SMatrix::from_raw(MATRIX);
    let out = graph.spectral_layout::<3>(1e-8);

    assert!(VectorOp::dot(&out[0], &out[1]).abs() < 1e-8);
    assert!(VectorOp::dot(&out[0], &out[2]).abs() < 1e-8);
    assert!(VectorOp::dot(&out[1], &out[2]).abs() < 1e-8);
}
