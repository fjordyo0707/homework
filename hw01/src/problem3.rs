pub mod p3 {
    /// Find all prime numbers less than `n`.
    /// For example, `sieve(7)` should return `[2, 3, 5]`
    pub fn sieve(n: u32) -> Vec<u32> {
        let mut s_o_e = vec![true; (n+1) as usize ];
        let mut ans : Vec<u32> = Vec::new();

        for idx in 2..n+1 {
            if s_o_e[idx as usize] {
                ans.push(idx);
                let mut mul_i = idx;
                while mul_i * idx <= n {
                    s_o_e[(mul_i * idx) as usize] = false;
                    mul_i += 1;
                }
            }
        }

        ans
    }
}