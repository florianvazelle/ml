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

        assert_eq!(&matrix1 * &matrix2, excepted);
    }

    #[test]
    fn transpose() {
        let mut matrix = Matrix::new(2, 3);
        matrix.data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

        let mut excepted = Matrix::new(3, 2);
        excepted.data = vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0];

        assert_eq!(matrix.transpose(), excepted);
    }

    #[test]
    fn transpose_vec() {
        let mut matrix = Matrix::new(1, 3);
        matrix.data = vec![1.0, 2.0, 3.0];

        let mut excepted = Matrix::new(3, 1);
        excepted.data = vec![1.0, 2.0, 3.0];

        assert_eq!(matrix.transpose(), excepted);
    }

    #[test]
    fn colum() {
        let mut matrix = Matrix::new(3, 3);
        matrix.data = vec![1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0];

        let mut excepted = Matrix::new(1, 3);

        excepted.data = vec![1.0, 1.0, 0.0];
        assert_eq!(matrix.column(0), excepted);
        excepted.data = vec![1.0, 0.0, 1.0];
        assert_eq!(matrix.column(1), excepted);
        excepted.data = vec![0.0, 1.0, 1.0];
        assert_eq!(matrix.column(2), excepted);
    }

    #[test]
    fn colum_mul() {
        let mut matrix = Matrix::new(3, 3);
        matrix.data = vec![1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0];

        let mut excepted = Matrix::new(1, 1);
        excepted.data = vec![2.0];

        assert_eq!(
            &matrix.column(0) * &(matrix.column(0).transpose()),
            excepted
        );
    }

    #[test]
    fn round() {
        let mut matrix = Matrix::new(2, 2);
        matrix.data = vec![0.12345, 0.999, 0.1257, 0.1200];

        let mut excepted = Matrix::new(2, 2);
        excepted.data = vec![0.12, 1.0, 0.13, 0.12];

        assert_eq!(matrix.round(2), excepted);
    }

    #[test]
    fn column_stack() {
        let mut matrix1 = Matrix::new(3, 1);
        matrix1.data = vec![1.0, 2.0, 3.0];

        let mut matrix2 = Matrix::new(3, 1);
        matrix2.data = vec![4.0, 5.0, 6.0];

        let mut excepted = Matrix::new(3, 2);
        excepted.data = vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0];

        assert_eq!(matrix1.column_stack(&matrix2), excepted);
    }

    #[test]
    fn qr_1() {
        let mut matrix = Matrix::new(3, 3);
        matrix.data = vec![1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0];

        let (q, r) = matrix.qr();
        // println!("Q = \n{}", q);
        println!("R = \n{}", r);

        let mut expected_q = Matrix::new(3, 3);
        expected_q.data = vec![
            0.707107, 0.408248, -0.57735, 0.707107, -0.408248, 0.57735, 0.0, 0.816497, 0.57735,
        ];

        let mut expected_r = Matrix::new(3, 3);
        expected_r.data = vec![
            1.414214, 0.707107, 0.707107, 0.0, 1.224745, 0.408248, 0.0, 0.0, 1.154701,
        ];

        assert_eq!(q.round(6), expected_q);
        assert_eq!(r.round(6), expected_r);
    }

    #[test]
    fn qr_2() {
        let mut matrix = Matrix::new(2, 3);
        matrix.data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        println!("{}", matrix);

        let (q, r) = matrix.qr();
        // println!("Q = \n{}", q);
        println!("R = \n{}", r);

        let mut expected_q = Matrix::new(2, 3);
        expected_q.data = vec![0.242536, 0.970143, 0.894427, 0.970143, -0.242536, -0.447214];

        let mut expected_r = Matrix::new(3, 3);
        expected_r.data = vec![
            4.123106, 5.335784, 6.548462, -0.0, 0.727607, 1.455214, -0.894427, -0.447214, 0.0,
        ];

        assert_eq!(q.round(6), expected_q);
        assert_eq!(r.round(6), expected_r);
    }
}
