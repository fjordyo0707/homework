pub mod p1{
    /// Computes the sum of all elements in the input i32 slice named `slice`
    pub fn sum(slice: &[i32]) -> i32 {
        let mut sum_slice = 0;
        for x in slice {
            sum_slice += x;
        }
        sum_slice
    }

    /// Deduplicates items in the input vector `vs`. Produces a vector containing
    /// the first instance of each distinct element of `vs`, preserving the
    /// original order.
    pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
        let mut de_vs : Vec<i32> = Vec::new();
        for x in vs.iter() {
            if !de_vs.contains(&x) {
                de_vs.push(*x);
            }
        }
        de_vs
    }

    /// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
    /// `bool`). Returns a new vector containing only elements that satisfy `pred`.
    pub fn filter(vs: &Vec<i32>, pred: & dyn Fn(i32) -> bool) -> Vec<i32> {
        let mut filter_vs : Vec<i32> = Vec::new();
        for x in vs.iter() {
            if pred(*x) {
                filter_vs.push(*x);
            }
        }
        filter_vs
    }
}
    