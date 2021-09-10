use crate::matrix::Matrix;

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    #[inline]
    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    #[inline]
    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    pub fn column(&self, j: usize) -> Matrix {
        let (rows, _) = self.shape();

        let mut data = Vec::with_capacity(rows);
        for i in 0..rows {
            data.push(self[(i, j)]);
        }

        Matrix {
            rows,
            cols: 1,
            data,
        }
    }

    pub fn round(&self, decimal: i32) -> Matrix {
        let (rows, cols) = self.shape();
        let factor = 10.0_f64.powi(decimal);
        let data = self.iter().map(|l| (l * factor).round() / factor).collect();

        Matrix { rows, cols, data }
    }

    pub fn sqrt(&self) -> Matrix {
        let (rows, cols) = self.shape();
        let data = self.iter().map(|l| l.sqrt()).collect();

        Matrix { rows, cols, data }
    }

    /// Transposes `self` and store the result into `out`.
    pub fn transpose(&self) -> Matrix {
        let mut out = Matrix::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                out[(j, i)] = self[(i, j)];
            }
        }
        out
    }

    /// Add a column
    pub fn column_stack(&self, other: &Self) -> Matrix {
        let mut mat1 = self.clone();
        let mut mat2 = other.clone();

        if mat1.rows < 2 {
            mat1 = mat1.transpose();
        }

        if mat2.rows < 2 {
            mat2 = mat2.transpose();
        }

        let mut product = Matrix::new(mat1.rows, mat1.cols + 1);
        for i in 0..mat1.rows {
            for j in 0..mat1.cols {
                product[(i, j)] = mat1[(i, j)];
            }
            product[(i, mat1.cols)] = mat2[(i, 0)];
        }
        product
    }
}
