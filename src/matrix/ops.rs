///! Operator overload.
use std::ops;

use crate::matrix::Matrix;

impl_op!(+|lhs: &Matrix, rhs: &Matrix| -> Matrix {
    assert_eq!(lhs.shape(), rhs.shape(), "Matrix addition dimensions mismatch {:?} and {:?}.", lhs.shape(), rhs.shape());

    let (rows, cols) = lhs.shape();
    let data = lhs.iter().zip(rhs.iter()).map(|(l, r)| l + r).collect();

    Matrix { rows, cols, data }
});

impl_op!(-|lhs: &Matrix, rhs: &Matrix| -> Matrix {
    assert_eq!(
        lhs.shape(),
        rhs.shape(),
        "Matrix subtraction dimensions mismatch {:?} and {:?}.",
        lhs.shape(),
        rhs.shape()
    );

    let (rows, cols) = lhs.shape();
    let data = lhs.iter().zip(rhs.iter()).map(|(l, r)| l - r).collect();

    Matrix { rows, cols, data }
});

impl_op!(*|lhs: &Matrix, rhs: &Matrix| -> Matrix {
    let (rows, lcols) = lhs.shape();
    let (rrows, cols) = rhs.shape();

    // if lcols != rrows && lcols == cols {
    //     return lhs * &rhs.transpose();
    // }

    assert_eq!(
        lcols,
        rrows,
        "Matrix multiplication dimensions mismatch {:?} and {:?}.",
        lhs.shape(),
        rhs.shape()
    );

    let mut data = Vec::with_capacity(rows * cols);
    for n in 0..rows {
        for m in 0..cols {
            let mut sum = 0.0;

            for k in 0..lcols {
                #[allow(clippy::suspicious_arithmetic_impl)]
                {
                    sum += lhs[(n, k)] * rhs[(k, m)];
                }
            }

            data.push(sum);
        }
    }

    Matrix { rows, cols, data }
});

impl_op!(/|lhs: &Matrix, rhs: &f64| -> Matrix {
    let (rows, cols) = lhs.shape();
    let data = lhs.iter().map(|l| *l / rhs).collect();

    Matrix { rows, cols, data }
});
