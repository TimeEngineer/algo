use super::matrix::SDMatrix;
use core::cmp::PartialEq;
use std::collections::VecDeque;

impl<T: Default + PartialEq> SDMatrix<T> {
    pub fn bfs(&self, v: usize) -> bool {
        let mut not_visited = vec![true; self.n];
        let mut queue = VecDeque::with_capacity(self.n);
        queue.push_back(v);

        while let Some(vi) = queue.pop_front() {
            not_visited[vi] = false;
            (0..self.n)
                .into_iter()
                .filter(|&vj| self[vi][vj] != T::default() && not_visited[vj])
                .for_each(|vj| queue.push_back(vj));
        }

        !not_visited.into_iter().any(|vi| vi)
    }
}
