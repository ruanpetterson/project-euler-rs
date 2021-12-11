pub mod problem_0001;
pub mod problem_0002;
pub mod problem_0003;
pub mod problem_0007;

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

    #[test]
    fn problem_0003() {
        use crate::problem_0003::is_prime;
        use crate::problem_0003::Prime;

        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(6));
        assert!(is_prime(7));
        assert!(!is_prime(10));
        assert!(is_prime(11));
        assert!(!is_prime(27));
        assert!(is_prime(29));

        assert_eq!(
            Prime::default()
                .take_while(|&n| n <= 29)
                .collect::<Vec<usize>>(),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        );
    }
}
