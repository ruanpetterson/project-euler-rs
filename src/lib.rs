pub mod problem_0001;
pub mod problem_0002;

#[cfg(test)]
mod tests {

    #[test]
    fn problem_0001() {
        use crate::problem_0001::multiples_of_3_or_5;

        let m = multiples_of_3_or_5(10);
        assert_eq!(m, vec![3usize, 5, 6, 9]);
        assert_eq!(m.into_iter().sum::<usize>(), 23);
    }

    #[test]
    fn problem_0002() {
        use crate::problem_0002::Fibonacci;

        assert_eq!(
            Fibonacci::default().take(10).collect::<Vec<usize>>(),
            vec![1usize, 2, 3, 5, 8, 13, 21, 34, 55, 89]
        );
    }
}
