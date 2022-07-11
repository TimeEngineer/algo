use super::matrix::SMatrix;
use core::cmp::PartialEq;

impl<T: Default + PartialEq, const N: usize> SMatrix<T, N> {
    pub fn dfs(&self, v: usize) -> bool {
        let mut not_visited = [true; N];
        let mut stack = Vec::with_capacity(N);
        stack.push(v);

        while let Some(vi) = stack.pop() {
            not_visited[vi] = false;
            (0..N)
                .into_iter()
                .filter(|&vj| self[vi][vj] != T::default() && not_visited[vj])
                .for_each(|vj| stack.push(vj));
        }

        !not_visited.into_iter().any(|vi| vi)
    }
}

#[test]
fn dfs_connected() {
    const ROW0: [f64; 5] = [0., 1., 1., 1., 1.];
    const ROW1: [f64; 5] = [1., 0., 1., 1., 1.];
    const ROW2: [f64; 5] = [1., 1., 0., 1., 1.];
    const ROW3: [f64; 5] = [1., 1., 1., 0., 1.];
    const ROW4: [f64; 5] = [1., 1., 1., 1., 0.];
    const MATRIX: [[f64; 5]; 5] = [ROW0, ROW1, ROW2, ROW3, ROW4];

    let graph = SMatrix::from_raw(MATRIX);

    assert_eq!(graph.dfs(0), true);
}

#[test]
fn dfs_not_connected() {
    const ROW0: [f64; 5] = [0., 0., 0., 0., 0.];
    const ROW1: [f64; 5] = [0., 0., 0., 0., 0.];
    const ROW2: [f64; 5] = [0., 0., 0., 0., 0.];
    const ROW3: [f64; 5] = [0., 0., 0., 0., 0.];
    const ROW4: [f64; 5] = [0., 0., 0., 0., 0.];
    const MATRIX: [[f64; 5]; 5] = [ROW0, ROW1, ROW2, ROW3, ROW4];

    let graph = SMatrix::from_raw(MATRIX);

    assert_eq!(graph.dfs(0), false);
}
