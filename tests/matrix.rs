#[cfg(test)]
mod tests {
    use ml::matrix::Matrix;

    #[test]
    fn mul() {
        let mut matrix1 = Matrix::new(3, 2);
        matrix1.data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

        let mut matrix2 = Matrix::new(2, 4);
        matrix2.data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

        let mut excepted = Matrix::new(3, 4);
        excepted.data = vec![
            11.0, 14.0, 17.0, 20.0, 23.0, 30.0, 37.0, 44.0, 35.0, 46.0, 57.0, 68.0,
        ];

        assert_eq!(matrix1.mul(&matrix2), excepted);
    }

    #[test]
    fn transpose() {
        let mut matrix = Matrix::new(3, 3);
        matrix.data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];

        let mut excepted = Matrix::new(3, 3);
        excepted.data = vec![1.0, 4.0, 7.0, 2.0, 5.0, 8.0, 3.0, 6.0, 9.0];

        assert_eq!(matrix.transpose(), excepted);
    }
}
