use ml::matrix::Matrix;
use ml::traits::Model;

use rand::prelude::*;

pub struct LinearRegression {
    weights: Vec<f64>,
    weights_count: usize,
}

impl LinearRegression {
    // Create a default linear regression model
    pub fn new(weights_count: usize) -> LinearRegression {
        let mut rng = rand::thread_rng();

        // Init all weights and biases between -1.0 and 1.0
        let mut weights: Vec<f64> = Vec::with_capacity(weights_count + 1);
        for i in 0..weights_count + 1 {
            let random: f64 = rng.gen(); // generates a float between 0 and 1
            weights[i] = 2.0 * random - 1.0;
        }

        LinearRegression {
            weights,
            weights_count,
        }
    }
}

impl Model for LinearRegression {
    fn train(&mut self, train_inputs: &Matrix, train_outputs: &Matrix) {
        // Add a column of one (at the right), for the bias
        let mut tmp = Matrix::new(train_inputs.rows, train_inputs.cols + 1);

        for i in 0..train_inputs.rows {
            for j in 0..train_inputs.cols {
                tmp.set(i, j, train_inputs.get(i, j));
            }
            tmp.set(i, train_inputs.cols, 1.0);
        }

        // Compute the transpose
        let inputs_transposed = tmp.transpose();
        let inv_inputs_transposed = inputs_transposed.mul(&tmp);
        // .completeOrthogonalDecomposition()
        // .pseudoInverse();

        // Compute weights
        let w = inv_inputs_transposed
            .mul(&inputs_transposed)
            .mul(&train_outputs);

        for i in 0..train_inputs.cols {
            self.weights[i] = w.get(i, 0);
        }
    }

    fn predict(&self, inputs: &Matrix, outputs: &mut Matrix) {
        assert_eq!(inputs.rows, outputs.rows); // or maybe resize outputs

        // for each sample
        for i in 0..inputs.rows {
            // Loop on outputs (here we have only one output)
            for j in 0..outputs.cols {
                let mut activation = self.weights[self.weights_count];

                // Loop on inputs
                for k in 0..inputs.cols {
                    activation += inputs.get(i, k) * self.weights[k];
                }

                // Compute the _sigmoid of the activation
                outputs.set(i, j, activation);
            }
        }
    }
}
