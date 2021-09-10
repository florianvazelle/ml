use std::ops;

use crate::matrix::Matrix;

impl ops::Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, ij: (usize, usize)) -> &f64 {
        let (rows, cols) = self.shape();

        assert!(ij.0 < rows && ij.1 < cols, "Matrix index out of bounds.");

        &self.data[ij.0 * self.cols + ij.1]
    }
}

impl ops::IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, ij: (usize, usize)) -> &mut f64 {
        let (rows, cols) = self.shape();

        assert!(ij.0 < rows && ij.1 < cols, "Matrix index out of bounds.");

        &mut self.data[ij.0 * self.cols + ij.1]
    }
}
