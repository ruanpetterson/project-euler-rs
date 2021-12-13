use crate::Solution;

pub struct Problem;

impl Solution for Problem {
    #[inline]
    fn solve() {
        let answer = (20usize..)
            .step_by(20)
            .find(|&n| (1..20).all(|m| n % m == 0))
            .unwrap();

        println!("{}", answer);
    }
}
