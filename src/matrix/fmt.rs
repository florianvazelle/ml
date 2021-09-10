use std::fmt;

use crate::matrix::Matrix;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut repr = String::new();
        for i in 0..self.rows {
            if i != 0 {
                repr += ",\n ";
            }
            repr += "[";
            for j in 0..self.cols {
                if j != 0 {
                    repr += ", ";
                }
                repr += format!("{}", self[(i, j)]).as_str();
            }
            repr += "]";
        }
        write!(f, "[{}]", repr)
    }
}
