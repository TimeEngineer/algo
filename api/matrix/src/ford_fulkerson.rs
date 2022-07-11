use crate::SMatrix;
use core::ops::{AddAssign, SubAssign};
use std::collections::VecDeque;

impl<T: Default + PartialEq, const N: usize> SMatrix<T, N> {
    fn _bfs(&self, src: usize, dst: usize, parent: &mut [usize; N]) -> bool {
        let mut not_visited = [true; N];
        let mut stack = VecDeque::with_capacity(N);
        stack.push_back(src);

        while let Some(vi) = stack.pop_front() {
            not_visited[vi] = false;
            for vj in (0..N)
                .into_iter()
                .filter(|&vj| self[vi][vj] != T::default() && not_visited[vj])
            {
                stack.push_back(vj);
                parent[vj] = vi;
                if vj == dst {
                    return true;
                }
            }
        }
        false
    }
    fn _dfs(&self, src: usize, dst: usize, parent: &mut [usize; N]) -> bool {
        let mut not_visited = [true; N];
        let mut stack = Vec::with_capacity(N);
        stack.push(src);

        while let Some(vi) = stack.pop() {
            not_visited[vi] = false;
            for vj in (0..N)
                .into_iter()
                .filter(|&vj| self[vi][vj] != T::default() && not_visited[vj])
            {
                stack.push(vj);
                parent[vj] = vi;
                if vj == dst {
                    return true;
                }
            }
        }
        false
    }
}

impl<T: Default + Copy + PartialOrd + AddAssign + SubAssign, const N: usize> SMatrix<T, N> {
    pub fn ford_fulkerson(&mut self, src: usize, dst: usize) -> T {
        let mut flow = T::default();
        let mut parent = [0; N];

        while self._bfs(0, N - 1, &mut parent) {
            let mut path_flow = T::default();
            let mut v = dst;

            while v != src {
                let u = parent[v];
                let w = self[u][v];
                if path_flow == T::default() || path_flow > w {
                    path_flow = w;
                }
                v = u;
            }

            flow += path_flow;

            let mut v = dst;

            while v != src {
                let u = parent[v];
                self[u][v] -= path_flow;
                self[v][u] += path_flow;
                v = u;
            }
        }
        flow
    }
}

#[test]
fn ford_fulkerson() {
    const ROW0: [u8; 4] = [0, 4, 2, 0];
    const ROW1: [u8; 4] = [0, 0, 3, 1];
    const ROW2: [u8; 4] = [0, 0, 0, 6];
    const ROW3: [u8; 4] = [0, 0, 0, 0];
    const MATRIX: [[u8; 4]; 4] = [ROW0, ROW1, ROW2, ROW3];

    let mut graph = SMatrix::from_raw(MATRIX);

    assert_eq!(graph.ford_fulkerson(0, 3), 6);
}
