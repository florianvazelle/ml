use std::fmt;

#[derive(Debug)]
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

    pub fn set(&mut self, i: usize, j: usize, val: f64) {
        let idx = i * self.cols + j;
        if idx >= self.data.len() {
            panic!("Out of range")
        }
        self.data[idx] = val
    }

    pub fn get(&self, i: usize, j: usize) -> f64 {
        let idx = i * self.cols + j;
        if idx >= self.data.len() {
            panic!("Out of range")
        }
        self.data[idx]
    }

    pub fn mul(&self, other: &Self) -> Matrix {
        assert_eq!(self.cols, other.rows);

        let mut product = Matrix::new(self.rows, other.cols);
        for n in 0..self.rows {
            for m in 0..other.cols {
                let mut sum = 0.0;

                for k in 0..self.cols {
                    println!("{} {} {}", n, k, m);
                    sum += self.get(n, k) * other.get(k, m);
                }

                product.set(n, m, sum);
            }
        }
        product
    }

    pub fn transpose(&self) -> Matrix {
        assert_eq!(self.rows, self.cols);

        let mut transpose = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                transpose.set(j, i, self.get(i, j));
            }
        }
        transpose
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.rows != other.rows || self.cols != other.cols {
            return false;
        }

        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.get(i, j) != other.get(i, j) {
                    return false;
                }
            }
        }
        true
    }
}

impl Eq for Matrix {}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut repr = String::new();
        for i in 0..self.rows {
            repr += "(";
            for j in 0..self.cols {
                repr += format!("{}, ", self.get(i, j)).as_str();
            }
            repr += ")\n";
        }
        write!(f, "{}", repr)
    }
}
