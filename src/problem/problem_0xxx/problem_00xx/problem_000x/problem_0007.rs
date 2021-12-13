use crate::problem::problem_0xxx::problem_00xx::problem_0003::Prime;
use crate::Solution;

pub struct Problem;

impl Solution for Problem {
    #[inline]
    fn solve() {
        println!("{}", Prime::default().into_iter().nth(10_001 - 1).unwrap());
    }
}
