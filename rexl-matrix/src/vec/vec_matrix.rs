/// row based
pub struct VecMatrix<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<T>>,
}

impl VecMatrix<f64> {
    pub fn new(rows: usize, cols: usize) -> VecMatrix<f64> {
        VecMatrix { rows, cols, data: vec![vec![0.; cols]; rows] }
    }

    pub fn pascal(level: usize) -> VecMatrix<f64> {
        let mut this = VecMatrix::new(level, level);
        for i in 0..level {
            for j in 0..level {
                if i == 0 || j == 0 {
                    this.data[i][j] = 1.
                } else {
                    this.data[i][j] = this.data[i - 1][j] + this.data[i][j - 1];
                }
            }
        }
        this
    }

    pub fn swap_row(&mut self, i1: usize, i2: usize) {
        self.data.swap(i1, i2);
    }

    pub fn add_other_row(&mut self, dest: usize, src: usize, multiply: f64) {
        for i in 0..self.cols {
            self.data[dest][i] += self.data[src][i] * multiply;
        }
    }

    pub fn multiply_row(&mut self, dest: usize, multiply: f64) {
        for i in 0..self.cols {
            self.data[dest][i] *= multiply;
        }
    }

    pub fn diagonalize(&mut self) {
        let mut dimension = self.rows;
        if self.rows > self.cols {
            dimension = self.cols
        }

        let mut sign = true;
        for k in 0..dimension - 1 {
            if self.data[k][k] == 0. {
                let mut swapped = false;
                // swap rows to make it no-zero
                for i in (k + 1)..dimension {
                    if self.data[i][k] != 0. {
                        self.swap_row(k, i);
                        swapped = true;
                        // reverse sign
                        sign = !sign;
                        break;
                    }
                }
                // cannot be diagonalized
                if !swapped {
                    return;
                }
            }

            for i in (k + 1)..dimension {
                self.add_other_row(i, k, -self.data[i][k] / self.data[k][k]);
            }
        }

        if !sign {
            self.multiply_row(0, -1.);
        }
    }

    pub fn det(&mut self) -> f64 {
        let mut dimension = self.rows;
        if self.rows > self.cols {
            dimension = self.cols
        }

        self.diagonalize();
        let mut result = 1.;
        for i in 0..dimension {
            result *= self.data[i][i];
        }
        result
    }

    pub fn print_data(&self) {
        for row in &self.data {
            println!("{:?}", row);
        }
    }
}