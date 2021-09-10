use crate::matrix::Matrix;

impl Matrix {
    /// Compute the qr factorization of a matrix.
    pub fn qr(&self) -> (Matrix, Matrix) {
        let mut q = Matrix::new(self.rows, 1);

        let first_column = self.column(0);

        // Compute the length of the i column of q
        let length = (&first_column.transpose() * &first_column)[(0, 0)].sqrt();

        // Normalize the first column of q
        for i in 0..self.rows {
            q[(i, 0)] = first_column[(i, 0)] / length;
        }

        for j in 1..self.cols {
            // If self is 3x3 matrix

            let current_column = self.column(j); // colum is 3x1

            let u = &q * &(&q.transpose() * &current_column); // 3x1 => 1x3 (transpose)
                                                              // 1x3 . 3x1 => 1x1 (mul with column)
                                                              // 3x1 . 1x1 => 3x1 (mul)
                                                              // ---
                                                              // 3x2 => 2x3 (transpose)
                                                              // 2x3 . 3x1 => 2x1 (mul with column)
                                                              // 3x2 . 2x1 => 3x1 (mul)

            let u = &current_column - &u;
            let e = (&u.transpose() * &u)[(0, 0)].sqrt(); // When you multiply a vector by itself,
                                                          // you must always transpose the left
                                                          // operand to obtain a unique value

            let tmp = &u / &e;
            q = q.column_stack(&tmp);
        }

        let r = &q.transpose() * self;

        (q, r)
    }
}
