use crate::matrix::Matrix;

impl PartialEq for Matrix {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.shape() == rhs.shape() && self.iter().zip(rhs.iter()).all(|(l, r)| l == r)
    }
}

impl Eq for Matrix {}
