use super::matrix::SDMatrix;
use core::cmp::PartialEq;

impl<T: Default + PartialEq> SDMatrix<T> {
    pub fn dfs(&self, v: usize) -> bool {
        let mut not_visited = vec![true; self.n];
        let mut stack = Vec::with_capacity(self.n);
        stack.push(v);

        while let Some(vi) = stack.pop() {
            not_visited[vi] = false;
            (0..self.n)
                .into_iter()
                .filter(|&vj| self[vi][vj] != T::default() && not_visited[vj])
                .for_each(|vj| stack.push(vj));
        }

        !not_visited.into_iter().any(|vi| vi)
    }
}
