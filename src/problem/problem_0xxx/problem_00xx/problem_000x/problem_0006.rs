use crate::Solution;

pub struct Problem;

impl Solution for Problem {
    fn solve() {
        let a = (1u32..=100).sum::<u32>().pow(2);
        let b = (1u32..=100).map(|n| n.pow(2)).sum::<u32>();
        println!("{} - {} = {}", a, b, a - b);
    }
}
