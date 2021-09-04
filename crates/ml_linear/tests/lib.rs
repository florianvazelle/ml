#[cfg(test)]
mod tests {
    use ml::matrix::Matrix;
    use ml::traits::Model;
    use ml_linear::LinearRegression;

    #[test]
    fn linear_simple_2d() {
        let mut inputs = Matrix::new(2, 1);
        inputs.data = vec![1.0, 2.0];

        let mut outputs = Matrix::new(2, 1);
        outputs.data = vec![2.0, 3.0];

        let mut model = LinearRegression::new(1);
        model.train(&inputs, &outputs);

        let mut results = Matrix::new(2, 1);
        model.predict(&inputs, &mut results);

        assert_eq!(outputs, results);
    }

    #[test]
    fn linear_simple_3d() {
        let mut inputs = Matrix::new(3, 2);
        inputs.data = vec![1.0, 1.0, 2.0, 2.0, 3.0, 1.0];

        let mut outputs = Matrix::new(3, 1);
        outputs.data = vec![2.0, 3.0, 2.5];

        let mut model = LinearRegression::new(2);
        model.train(&inputs, &outputs);

        let mut results = Matrix::new(3, 1);
        model.predict(&inputs, &mut results);

        assert_eq!(outputs, results);
    }

    #[test]
    fn linear_tricky_3d() {
        let mut inputs = Matrix::new(3, 2);
        inputs.data = vec![1.0, 1.0, 2.0, 2.0, 3.0, 3.0];

        let mut outputs = Matrix::new(3, 1);
        outputs.data = vec![1.0, 2.0, 3.0];

        let mut model = LinearRegression::new(2);
        model.train(&inputs, &outputs);

        let mut results = Matrix::new(3, 1);
        model.predict(&inputs, &mut results);

        assert_eq!(outputs, results);
    }
}
