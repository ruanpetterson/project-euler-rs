use crate::Solution;

#[inline]
fn multiples_of_3_or_5(max: usize) -> Vec<usize> {
    (1..max)
        .into_iter()
        .filter(|&n| n % 3 == 0 || n % 5 == 0)
        .collect()
}

pub struct Problem;

impl Solution for Problem {
    #[inline]
    fn solve() {
        const INPUT: usize = 1000;

        println!("{}", multiples_of_3_or_5(INPUT).into_iter().sum::<usize>());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sum_of_all_multiples_of_3_or_5_less_than_10() {
        use super::multiples_of_3_or_5;

        let m = multiples_of_3_or_5(10);
        assert_eq!(m, vec![3usize, 5, 6, 9]);
        assert_eq!(m.into_iter().sum::<usize>(), 23);
    }
}
