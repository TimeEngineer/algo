/// Simplex is not finished yet
pub struct Lp {
    pub table: Vec<f64>,
    pub nrow: usize,
    pub ncol: usize,
}

impl std::fmt::Debug for Lp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(precision) = f.precision() {
            for row in self.table.chunks(self.ncol) {
                writeln!(f, "{row:.*?}", precision)?
            }
        } else {
            for row in self.table.chunks(self.ncol) {
                writeln!(f, "{row:?}")?
            }
        }
        Ok(())
    }
}

impl Lp {
    fn pivot_bland(&self) -> (usize, usize) {
        let j = self
            .table
            .iter()
            .take(self.ncol)
            .enumerate()
            .skip(1)
            .find(|(_, v)| **v > 1e-15)
            .unwrap()
            .0;

        let i = self
            .table
            .chunks(self.ncol)
            .enumerate()
            .skip(1)
            .filter(|(_, v)| v[j] > 1e-15)
            .min_by(|(_, a), (_, b)| (a[0] / a[j]).partial_cmp(&(b[0] / b[j])).unwrap())
            .unwrap()
            .0;

        (i, j)
    }

    fn reduce(&mut self, i: usize, j: usize) {
        let (part0, part1) = self.table.split_at_mut(i * self.ncol);
        let (row, part1) = part1.split_at_mut(self.ncol);

        let x = 1. / row[j];
        row.iter_mut().for_each(|v| *v *= x);

        part0.chunks_mut(self.ncol).for_each(|_row| {
            let x = _row[j];
            _row.iter_mut()
                .zip(row.iter())
                .for_each(|(_v, v)| *_v -= v * x)
        });
        part1.chunks_mut(self.ncol).for_each(|_row| {
            let x = _row[j];
            _row.iter_mut()
                .zip(row.iter())
                .for_each(|(_v, v)| *_v -= v * x)
        });
    }

    fn is_optimal(&self) -> bool {
        self.table.iter().take(self.ncol).all(|v| *v < 1e-15)
    }

    pub fn solve(&mut self) {
        while !self.is_optimal() {
            let (i, j) = self.pivot_bland();
            // println!("pivot is {i}, {j}");
            self.reduce(i, j);
            // println!("{self:.2?}");
        }
    }
}

#[test]
fn simplex() {
    let mut lp = Lp {
        table: vec![
            0., 10., 6., 4., 0., 0., 0., // Z
            100., 1., 1., 1., 1., 0., 0., // C1
            600., 10., 4., 5., 0., 1., 0., // C2
            300., 2., 2., 6., 0., 0., 1., // C3
        ],
        nrow: 4,
        ncol: 7,
    };

    println!("{lp:.2?}");
    let now = std::time::Instant::now();
    lp.solve();
    println!("{:?}", now.elapsed())
}
