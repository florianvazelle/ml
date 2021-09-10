use std::slice;

use crate::matrix::Matrix;

impl Matrix {
    /// Iterates through this matrix coordinates in rows order
    #[inline]
    pub fn iter(&self) -> slice::Iter<'_, f64> {
        self.data.iter()
    }

    /// Mutably iterates through this matrix
    #[inline]
    pub fn iter_mut(&mut self) -> slice::IterMut<'_, f64> {
        self.data.iter_mut()
    }
}
