use crate::matrix::Matrix;

pub trait Model {
    fn train(&mut self, train_inputs: &Matrix, train_outputs: &Matrix);
    fn predict(&self, inputs: &Matrix, outputs: &mut Matrix);
}
