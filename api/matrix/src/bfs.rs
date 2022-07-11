use super::matrix::SMatrix;
use core::cmp::PartialEq;
use std::collections::VecDeque;

impl<T: Default + PartialEq, const N: usize> SMatrix<T, N> {
    pub fn bfs(&self, v: usize) -> bool {
        let mut not_visited = [true; N];
        let mut queue = VecDeque::with_capacity(N);
        queue.push_back(v);

        while let Some(vi) = queue.pop_front() {
            not_visited[vi] = false;
            (0..N)
                .into_iter()
                .filter(|&vj| self[vi][vj] != T::default() && not_visited[vj])
                .for_each(|vj| queue.push_back(vj));
        }

        !not_visited.into_iter().any(|vi| vi)
    }
}

#[test]
fn bfs_connected() {
    const ROW0: [f64; 5] = [0., 1., 1., 1., 1.];
    const ROW1: [f64; 5] = [1., 0., 1., 1., 1.];
    const ROW2: [f64; 5] = [1., 1., 0., 1., 1.];
    const ROW3: [f64; 5] = [1., 1., 1., 0., 1.];
    const ROW4: [f64; 5] = [1., 1., 1., 1., 0.];
    const MATRIX: [[f64; 5]; 5] = [ROW0, ROW1, ROW2, ROW3, ROW4];

    let graph = SMatrix::from_raw(MATRIX);

    assert_eq!(graph.bfs(0), true);
}

#[test]
fn bfs_not_connected() {
    const ROW0: [f64; 5] = [0., 0., 0., 0., 0.];
    const ROW1: [f64; 5] = [0., 0., 0., 0., 0.];
    const ROW2: [f64; 5] = [0., 0., 0., 0., 0.];
    const ROW3: [f64; 5] = [0., 0., 0., 0., 0.];
    const ROW4: [f64; 5] = [0., 0., 0., 0., 0.];
    const MATRIX: [[f64; 5]; 5] = [ROW0, ROW1, ROW2, ROW3, ROW4];

    let graph = SMatrix::from_raw(MATRIX);

    assert_eq!(graph.bfs(0), false);
}
