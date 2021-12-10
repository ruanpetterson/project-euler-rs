pub mod problem_0001;

#[cfg(test)]
mod tests {

    #[test]
    fn problem_0001() {
        use crate::problem_0001::multiples_of_3_or_5;

        let m = multiples_of_3_or_5(10);
        assert_eq!(m, vec![3usize, 5, 6, 9]);
        assert_eq!(m.into_iter().sum::<usize>(), 23);
    }
}
