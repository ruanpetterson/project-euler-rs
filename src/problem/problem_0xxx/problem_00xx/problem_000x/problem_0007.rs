use crate::problem::problem_0xxx::problem_00xx::problem_0003::Prime;
use crate::Solution;

pub struct Problem;

impl Solution for Problem {
    #[inline]
    fn solve() {
        const INPUT: usize = 10_001;

        println!("{}", Prime::default().into_iter().nth(INPUT - 1).unwrap());
    }
}
