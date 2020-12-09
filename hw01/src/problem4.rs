pub mod p4 {
    /// #[derive(...)] statements define certain properties on the enum for you for
    /// free (printing, equality testing, the ability to copy values). More on this
    /// when we cover Enums in detail.

    /// You can use any of the variants of the `Peg` enum by writing `Peg::B`, etc.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Peg {
        A,
        B,
        C,
    }

    /// A move between two pegs: (source, destination).
    pub type Move = (Peg, Peg);

    /// Solves for the sequence of moves required to move all discs from `src` to
    /// `dst`.
    pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
        if num_discs == 1 {
            return vec![(src, dst)];
        } else {
            let mut ans : Vec<Move> = Vec::new();
            let mut f_move = hanoi(num_discs - 1, src, dst, aux);
            let mut s_move = vec![(src, dst)];
            let mut t_move = hanoi(num_discs - 1, aux, src, dst);
            ans.append(&mut f_move);
            ans.append(&mut s_move);
            ans.append(&mut t_move);
            ans
        }
    }
}