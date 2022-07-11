use core::fmt::{Debug, Error, Formatter};
use core::ops::{Index, IndexMut, Mul};
use std::ops::AddAssign;

#[derive(Clone, Copy, PartialEq)]
pub struct Matrix<T, const N: usize, const M: usize>(pub [[T; M]; N]);
pub type SMatrix<T, const N: usize> = Matrix<T, N, N>;

impl<T, const N: usize, const M: usize> Index<usize> for Matrix<T, N, M> {
    type Output = [T; M];
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const N: usize, const M: usize> IndexMut<usize> for Matrix<T, N, M> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Default + Copy, const N: usize, const M: usize> Default for Matrix<T, N, M> {
    fn default() -> Self {
        Self([[T::default(); M]; N])
    }
}

impl<T, const N: usize, const M: usize> Matrix<T, N, M> {
    pub fn from_raw(data: [[T; M]; N]) -> Self {
        Self(data)
    }
    pub fn set(&mut self, i: usize, j: usize, value: T) {
        self[i][j] = value;
    }
}

impl<T: Debug, const N: usize, const M: usize> Debug for Matrix<T, N, M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if let Some(precision) = f.precision() {
            for row in self.0.iter() {
                writeln!(f, "{:.*?}", precision, row)?;
            }
        } else {
            for row in self.0.iter() {
                writeln!(f, "{:?}", row)?;
            }
        }
        Ok(())
    }
}

impl<T: Copy, const N: usize, const M: usize> Matrix<T, N, M> {
    pub fn sset(&mut self, i: usize, j: usize, value: T) {
        self.0[i][j] = value;
        self.0[j][i] = value;
    }
}

impl<
        T: Copy + Default + Mul<Output = T> + AddAssign,
        const K: usize,
        const N: usize,
        const M: usize,
    > Mul<&Matrix<T, K, M>> for &Matrix<T, N, K>
{
    type Output = Matrix<T, N, M>;
    fn mul(self, rhs: &Matrix<T, K, M>) -> Self::Output {
        let mut out = Self::Output::default();
        for i in 0..N {
            for k in 0..K {
                for j in 0..M {
                    out[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        out
    }
}

impl<T: Default + Copy + Mul<Output = T> + AddAssign, const N: usize, const M: usize> Mul<[T; M]>
    for &Matrix<T, N, M>
{
    type Output = [T; N];
    fn mul(self, rhs: [T; M]) -> Self::Output {
        let mut out = [T::default(); N];
        for i in 0..N {
            for (j, rhsj) in rhs.into_iter().enumerate() {
                out[i] += self[i][j] * rhsj;
            }
        }
        out
    }
}
