//! Matrix structure.

mod base;
mod cmp;
mod fmt;
mod idx;
mod iter;
mod linalg;
mod ops;

#[derive(Clone, Debug)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}
