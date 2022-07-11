use super::matrix::SMatrix;
use core::cmp::PartialOrd;
use core::ops::Add;

impl<T: Default + Copy + PartialOrd + Add<Output = T>, const N: usize> SMatrix<T, N> {
    fn visits(&self, visited: &mut [bool; N], distance: &mut [T; N]) -> (usize, T) {
        let mut min = (0, distance[0]);
        for d in distance.iter().copied().enumerate().skip(1) {
            if min.1 == T::default() || (d.1 != T::default() && d.1 < min.1) {
                min = d;
            }
        }
        visited[min.0] = true;
        distance[min.0] = T::default();
        min
    }
    fn update_distance(&self, cur: (usize, T), visited: &[bool; N], distance: &mut [T; N]) {
        for i in 0..N {
            let edge = self[cur.0][i];
            let distance_i = &mut distance[i];

            if !visited[i] {
                let new_dist = cur.1 + edge;
                if edge != T::default() && (*distance_i == T::default() || *distance_i > new_dist) {
                    *distance_i = new_dist;
                }
            }
        }
    }
    fn update_distance_with_path(
        &self,
        cur: (usize, T),
        visited: &[bool; N],
        distance: &mut [T; N],
        path: &mut [usize; N],
    ) {
        for i in 0..N {
            let edge = self[cur.0][i];
            let distance_i = &mut distance[i];
            let path_i = &mut path[i];

            if !visited[i] {
                let new_dist = cur.1 + edge;
                if edge != T::default() && (*distance_i == T::default() || *distance_i > new_dist) {
                    *distance_i = new_dist;
                    *path_i = cur.0;
                }
            }
        }
    }
    fn rebuild_path(&self, src: usize, dst: usize, path: &[usize; N]) -> Vec<usize> {
        let mut p = Vec::with_capacity(N);
        let mut last_vertex = dst;

        p.push(last_vertex);
        while last_vertex != src {
            p.push(path[last_vertex]);
            last_vertex = *p.last().unwrap();
        }
        p
    }
}

impl<T: Default + Copy + PartialOrd + Add<Output = T>, const N: usize> SMatrix<T, N> {
    pub fn dijkstra(&self, src: usize, dst: usize) -> T {
        let mut visited = [false; N];
        let mut distance = self[src];
        visited[src] = true;

        for _ in 0..(N - 1) {
            let cur = self.visits(&mut visited, &mut distance);
            self.update_distance(cur, &visited, &mut distance);
            if cur.0 == dst {
                return cur.1;
            }
        }
        T::default()
    }
    pub fn dijkstra_with_path(&self, src: usize, dst: usize) -> (T, Vec<usize>) {
        let mut visited = [false; N];
        let mut distance = self[src];
        let mut path = [src; N];
        visited[src] = true;

        for _ in 0..(N - 1) {
            let cur = self.visits(&mut visited, &mut distance);
            self.update_distance_with_path(cur, &visited, &mut distance, &mut path);
            if cur.0 == dst {
                return (cur.1, self.rebuild_path(src, dst, &path));
            }
        }
        (T::default(), self.rebuild_path(src, dst, &path))
    }
    pub fn dijkstra_from_src(&self, src: usize) -> Vec<T> {
        let mut output = vec![T::default(); N];
        let mut visited = [false; N];
        let mut distance = self[src];
        visited[src] = true;

        for _ in 0..(N - 1) {
            let cur = self.visits(&mut visited, &mut distance);
            self.update_distance(cur, &visited, &mut distance);
            output[cur.0] = cur.1;
        }
        output
    }
    pub fn dijkstra_from_src_with_path(&self, src: usize) -> Vec<(T, Vec<usize>)> {
        let mut output = Vec::with_capacity(N);
        let mut visited = [false; N];
        let mut distance = self[src];
        let mut path = [src; N];
        visited[src] = true;

        for _ in 0..(N - 1) {
            let cur = self.visits(&mut visited, &mut distance);
            self.update_distance_with_path(cur, &visited, &mut distance, &mut path);
            if cur.1 == T::default() {
                break;
            }
            output.push((cur.1, self.rebuild_path(src, cur.0, &path)))
        }
        output
    }
}

#[test]
fn dijkstra() {
    let mut graph = SMatrix::<f64, 10>::default();

    graph.sset(0, 1, 85.);
    graph.sset(0, 2, 217.);
    graph.sset(0, 4, 173.);
    graph.sset(1, 5, 80.);
    graph.sset(2, 6, 186.);
    graph.sset(2, 7, 103.);
    graph.sset(3, 7, 183.);
    graph.sset(4, 9, 502.);
    graph.sset(5, 8, 250.);
    graph.sset(7, 9, 167.);
    graph.sset(8, 9, 84.);

    assert_eq!(graph.dijkstra(0, 3), 503.);
}

#[test]
fn dijkstra_with_path() {
    let mut graph = SMatrix::<f64, 10>::default();

    graph.sset(0, 1, 85.);
    graph.sset(0, 2, 217.);
    graph.sset(0, 4, 173.);
    graph.sset(1, 5, 80.);
    graph.sset(2, 6, 186.);
    graph.sset(2, 7, 103.);
    graph.sset(3, 7, 183.);
    graph.sset(4, 9, 502.);
    graph.sset(5, 8, 250.);
    graph.sset(7, 9, 167.);
    graph.sset(8, 9, 84.);

    assert_eq!(graph.dijkstra_with_path(0, 3), (503., vec![3, 7, 2, 0]));
}

#[test]
fn dijkstra_from_src() {
    let mut graph = SMatrix::<f64, 10>::default();

    graph.sset(0, 1, 85.);
    graph.sset(0, 2, 217.);
    graph.sset(0, 4, 173.);
    graph.sset(1, 5, 80.);
    graph.sset(2, 6, 186.);
    graph.sset(2, 7, 103.);
    graph.sset(3, 7, 183.);
    graph.sset(4, 9, 502.);
    graph.sset(5, 8, 250.);
    graph.sset(7, 9, 167.);
    graph.sset(8, 9, 84.);

    assert_eq!(
        graph.dijkstra_from_src(0),
        vec![0., 85., 217., 503., 173., 165., 403., 320., 415., 487.]
    );
}

#[test]
fn dijkstra_from_src_with_path() {
    let mut graph = SMatrix::<f64, 10>::default();

    graph.sset(0, 1, 85.);
    graph.sset(0, 2, 217.);
    graph.sset(0, 4, 173.);
    graph.sset(1, 5, 80.);
    graph.sset(2, 6, 186.);
    graph.sset(2, 7, 103.);
    graph.sset(3, 7, 183.);
    graph.sset(4, 9, 502.);
    graph.sset(5, 8, 250.);
    graph.sset(7, 9, 167.);
    graph.sset(8, 9, 84.);

    assert_eq!(
        graph.dijkstra_from_src_with_path(0),
        vec![
            (85., vec![1, 0]),
            (165., vec![5, 1, 0]),
            (173., vec![4, 0]),
            (217., vec![2, 0]),
            (320., vec![7, 2, 0]),
            (403., vec![6, 2, 0]),
            (415., vec![8, 5, 1, 0]),
            (487., vec![9, 7, 2, 0]),
            (503., vec![3, 7, 2, 0]),
        ]
    );
}
