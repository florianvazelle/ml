use std::fmt;
use std::ops;
use std::slice;

#[derive(Clone, Debug)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn set(&mut self, i: usize, j: usize, val: f64) {
        let (rows, cols) = self.shape();
 
        assert!(
            i < rows && j < cols,
            "Matrix index out of bounds."
        );

        self.data[i * self.cols + j] = val
    }

    pub fn get(&self, i: usize, j: usize) -> f64 {
        let (rows, cols) = self.shape();
 
        assert!(
            i < rows && j < cols,
            "Matrix index out of bounds."
        );

        self.data[i * self.cols + j]
    }

    pub fn column(&self, j: usize) -> Matrix {
        let (rows, cols) = self.shape();
 
        assert!(
            j < cols,
            "Matrix index out of bounds."
        );

        let mut data = Vec::with_capacity(rows);
        for i in 0..rows {
            data.push(self.get(i, j));
        }
        
        Matrix { 
            rows: 1,
            cols,
            data
        }        
    }

    // Iterates through this matrix coordinates in column-major order
    #[inline]
    pub fn iter(&self) -> slice::Iter<'_, f64> {
        self.data.iter()
    }

    // Mutably iterates through this matrix
    #[inline]
    pub fn iter_mut(&mut self) -> slice::IterMut<'_, f64> {
        self.data.iter_mut()
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

    // Transposes `self` and store the result into `out`.
    pub fn transpose(&self) -> Matrix {
        let mut out = Matrix::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                out.set(j, i, self.get(i, j));
            }
        }
        out
    }

    // QR factorization
    pub fn qr(&self) -> (Matrix, Matrix) {
        let mut q = Matrix::new(1, self.cols);

        let first_column = self.column(0);

        // Compute the length of the i column of q
        let length = (&first_column * &first_column).get(0, 0).sqrt();

        // Normalize the first column of q
        for i in 0..self.cols {
            q.set(0, i, first_column.get(0, i) / length);
        }

        for j in 1..self.cols {
            // If self is 3x3 matrix

            let current_column = self.column(j); // colum is 1x3

            let u = &(&current_column * &q.transpose()) * &q; // 1x3 => 3x1 (transpose)
                                                          // 1x3 . 3x1 => 1x1 (mul with column)
                                                          // 1x1 . 1x3 => 1x3 (mul)
                                                          // ---
                                                          // 3x2 => 2x3 (transpose)
                                                          // 1x3 . 2x3 => 1x3 (mul with column)
                                                          // 1x3 . 1x3 => 1x3 (mul)

            let u = &current_column - &u;
            let e = (&u * &u).get(0, 0).sqrt();

            let tmp = &u / &e;
            q = q.column_stack(&tmp);
        }

        let r = (self * &q).transpose();

        (q, r)
    }

    // Add a column
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
                product.set(i, j, mat1.get(i, j));
            }
            product.set(i, mat1.cols, mat2.get(i, 0));
        }
        product
    }
}

impl PartialEq for Matrix {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.shape() == rhs.shape() && self.iter().zip(rhs.iter()).all(|(l, r)| l == r)
    }
}

impl Eq for Matrix {}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut repr = String::new();
        for i in 0..self.rows {
            if i != 0 {
                repr += ",\n";
            }
            repr += "[";
            for j in 0..self.cols {
                if j != 0 {
                    repr += ", ";
                }
                repr += format!("{}", self.get(i, j)).as_str();
            }
            repr += "]";
        }
        write!(f, "[{}]", repr)
    }
}

// Operator overload

impl_op!(+ |lhs: &Matrix, rhs: &Matrix| -> Matrix {
    assert_eq!(lhs.shape(), rhs.shape());

    let (rows, cols) = lhs.shape();
    let data = lhs.iter().zip(rhs.iter()).map(|(l, r)| l + r).collect();

    Matrix { rows, cols, data }
});

impl_op!(- |lhs: &Matrix, rhs: &Matrix| -> Matrix {
    assert_eq!(lhs.shape(), rhs.shape());

    let (rows, cols) = lhs.shape();
    let data = lhs.iter().zip(rhs.iter()).map(|(l, r)| l - r).collect();

    Matrix { rows, cols, data }
});

impl_op!(* |lhs: &Matrix, rhs: &Matrix| -> Matrix {
    let (rows, lcols) = lhs.shape();
    let (rrows, cols) = rhs.shape();

    if lcols != rrows && lcols == cols {
        return lhs * &rhs.transpose()
    }

    assert_eq!(lcols, rrows);

    let mut data = Vec::with_capacity(rows * cols);
    for n in 0..rows {
        for m in 0..cols {
            let mut sum = 0.0;

            for k in 0..lcols {
                sum += lhs.get(n, k) * rhs.get(k, m);
            }
            
            data.push(sum);
        }
    }

    Matrix { rows, cols, data }
});

impl_op!(/ |lhs: &Matrix, rhs: &f64| -> Matrix { 
    let (rows, cols) = lhs.shape();
    let data = lhs.iter().map(|l| *l / rhs).collect();

    Matrix { rows, cols, data }
});
