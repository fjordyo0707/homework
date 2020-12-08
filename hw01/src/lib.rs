pub mod problem1;
pub mod problem2;
pub mod problem3;
pub mod problem4;
mod tests_provided;

#[cfg(test)]
mod tests {
    use super::problem1::p1;
    use super::problem2::p2;
    #[test]
    fn test_dedup_small() {
        let vs = vec![1,2,2,3,4,1];
        assert_eq!(p1::dedup(&vs), vec![1,2,3,4]);
    }

    #[test]
    fn test_invalid_mat_mult() {
        let mat1 = vec![vec![0.;3]; 3];
        let mat2 = vec![vec![0.;4]; 4];
        let _mat3 = p2::mat_mult(&mat1, &mat2);
    }
}
