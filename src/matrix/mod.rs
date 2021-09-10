//! Matrix structure.

mod cmp;
mod fmt;
mod idx;
mod inv;
mod iter;
mod linalg;
mod matrix;
mod ops;

#[derive(Clone, Debug)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}
