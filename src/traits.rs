use super::{DatasetBase, Records};

pub trait Fit<'a, R: Records, T> {
    type Object: 'a;

    fn fit(&self, dataset: &DatasetBase<R, T>) -> Self::Object;
}

pub trait Predict<R: Records, T> {
    fn predict(&self, x: R) -> T;
}