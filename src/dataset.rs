pub trait Records: Sized {
    type Elem;

    fn nsamples(&self) -> usize;
    fn nfeatures(&self) -> usize;
}

pub struct DatasetBase<R, T>
where
    R: Records,
{
    pub records: R,
    pub targets: T,

    pub weights: Array1<f32>,
    feature_names: Vec<String>,
}
