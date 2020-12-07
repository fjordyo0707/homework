pub mod p2 {
    /// Represents a matrix in row-major order
    pub type Matrix = Vec<Vec<f32>>;

    /// Computes the product of the inputs `mat1` and `mat2`.
    pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
        assert!(mat1.len() > 0);
        assert!(mat1[0].len() > 0);
        assert!(mat2.len() > 0);
        assert!(mat2[0].len() > 0);
        assert_eq!(mat1[0].len(), mat2.len());

        let row_1 = mat1.len();
        let col_1 = mat1[0].len();
        let col_2 = mat2[0].len();
        let mut ans : Matrix = vec![ vec![0.; row_1]; col_2 ];
        for i in 0..row_1 {
            for j in 0..col_1 {
                for k in 0..col_2 {
                    ans[i][k] += mat1[i][j] * mat2[j][k];
                }
            }
        }
        return ans;
    }
}