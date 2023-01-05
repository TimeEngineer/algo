use super::matrix::Matrix;

impl<T: Copy + PartialEq, const N: usize, const M: usize> Matrix<T, N, M> {
    fn _flood_fill(&mut self, i: usize, j: usize, old_color: T, new_color: T) {
        let mut min_j = j;
        let mut max_j = j;

        self[i][j] = new_color;
        while min_j > 0 {
            min_j -= 1;
            if self[i][min_j] == old_color {
                self[i][min_j] = new_color;
            } else {
                break;
            }
        }
        while max_j < M - 1 {
            max_j += 1;
            if self[i][max_j] == old_color {
                self[i][max_j] = new_color;
            } else {
                break;
            }
        }

        for j in min_j..max_j {
            if i > 0 && self[i - 1][j] == old_color {
                self._flood_fill(i - 1, j, old_color, new_color);
            }
            if i < N - 1 && self[i + 1][j] == old_color {
                self._flood_fill(i + 1, j, old_color, new_color)
            }
        }
    }
    pub fn flood_fill(&mut self, i: usize, j: usize, new_color: T) {
        self._flood_fill(i, j, self[i][j], new_color)
    }
}

#[test]
fn flood_fill() {
    let mut m = Matrix::from_raw([[0; 4], [0, 1, 0, 0], [0; 4], [0; 4]]);
    let n = Matrix::from_raw([[2; 4], [2, 1, 2, 2], [2; 4], [2; 4]]);

    m.flood_fill(1, 0, 2);

    assert_eq!(m, n);
}
