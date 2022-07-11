use core::fmt::{Debug, Error, Formatter};
use core::ops::{Index, IndexMut};

#[derive(Default)]
pub struct DMatrix<T> {
    pub n: usize,
    pub m: usize,
    pub data: Vec<T>,
}

impl<T> Index<usize> for DMatrix<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        let n = self.m;
        &self.data[index * n..(index + 1) * n]
    }
}

impl<T> IndexMut<usize> for DMatrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let n = self.m;
        &mut self.data[index * n..(index + 1) * n]
    }
}

impl<T> DMatrix<T> {
    pub fn from_raw(data: Vec<T>, n: usize, m: usize) -> Self {
        Self { n, m, data }
    }
    pub fn set(&mut self, i: usize, j: usize, value: T) {
        self[i][j] = value;
    }
}

impl<T: Debug> Debug for DMatrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let n = self.n;
        if let Some(precision) = f.precision() {
            for i in 0..n {
                writeln!(f, "{:.*?}", precision, &self[i])?;
            }
        } else {
            for i in 0..n {
                writeln!(f, "{:?}", &self[i])?;
            }
        }
        Ok(())
    }
}

impl<T: Clone> DMatrix<T> {
    #[inline]
    pub fn sset(&mut self, i: usize, j: usize, value: T) {
        self[i][j] = value.clone();
        self[j][i] = value;
    }
}

#[derive(Default)]
pub struct SDMatrix<T> {
    pub n: usize,
    pub data: Vec<T>,
}

impl<T> Index<usize> for SDMatrix<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        let n = self.n;
        &self.data[index * n..(index + 1) * n]
    }
}

impl<T> IndexMut<usize> for SDMatrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let n = self.n;
        &mut self.data[index * n..(index + 1) * n]
    }
}

impl<T> SDMatrix<T> {
    pub fn from_raw(data: Vec<T>, n: usize) -> Self {
        Self { n, data }
    }
    #[inline]
    pub fn set(&mut self, i: usize, j: usize, value: T) {
        self[i][j] = value;
    }
}

impl<T: Debug> Debug for SDMatrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let n = self.n;
        if let Some(precision) = f.precision() {
            for i in 0..n {
                writeln!(f, "{:.*?}", precision, &self[i])?;
            }
        } else {
            for i in 0..n {
                writeln!(f, "{:?}", &self[i])?;
            }
        }
        Ok(())
    }
}

impl<T: Clone> SDMatrix<T> {
    #[inline]
    pub fn sset(&mut self, i: usize, j: usize, value: T) {
        self[i][j] = value.clone();
        self[j][i] = value;
    }
}
