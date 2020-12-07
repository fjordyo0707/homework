mod problem1;
pub mod problem2;
pub mod problem3;
pub mod problem4;
// mod tests_provided;
pub use crate::problem1::p1;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test_dedup_small() {
        let vs = vec![1,2,2,3,4,1];
        assert_eq!(p1::dedup(&vs), vec![1,2,3,4]);
    }
}
