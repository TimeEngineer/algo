use crate::SMatrix;

impl<T: Default + std::fmt::Debug + Copy + PartialEq + Ord, const N: usize> SMatrix<T, N> {
    pub fn kruskal(&self) -> Self {
        let mut stop = N - 1;
        let mut out = Self::default();
        let mut connect = [0; N];

        for (i, c) in connect.iter_mut().enumerate() {
            *c = i;
        }

        let mut edges = (0..N)
            .flat_map(|i| (0..N).map(move |j| (i, j)))
            .filter(|&(i, j)| self[i][j] != T::default())
            .map(|(i, j)| (i, j, self[i][j]))
            .collect::<Vec<_>>();

        edges.sort_unstable_by_key(|k| k.2);

        for e in edges {
            let c0 = connect[e.0];
            let c1 = connect[e.1];
            if c0 != c1 && out[e.0][e.1] == T::default() {
                out.set(e.0, e.1, e.2);
                if out[e.1][e.0] != T::default() {
                    connect
                        .iter_mut()
                        .filter(|c| **c == c1)
                        .for_each(|c| *c = c0);
                    stop -= 1;
                }
            }
            if stop == 0 {
                break;
            }
        }
        out
    }
}

#[test]
fn kruskal() {
    let mut graph = SMatrix::<u8, 5>::default();

    graph.sset(0, 1, 2);
    graph.sset(0, 3, 6);
    graph.sset(1, 2, 3);
    graph.sset(1, 3, 8);
    graph.sset(1, 4, 5);
    graph.sset(2, 4, 7);
    graph.sset(3, 4, 9);

    let mut expected = SMatrix::<u8, 5>::default();
    expected.sset(0, 1, 2);
    expected.sset(1, 2, 3);
    expected.sset(1, 4, 5);
    expected.sset(0, 3, 6);

    assert_eq!(graph.kruskal(), expected);
}
