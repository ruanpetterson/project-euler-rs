use crate::Solution;

pub struct Prime(usize);

impl Default for Prime {
    fn default() -> Self {
        Self(1)
    }
}

impl Iterator for Prime {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = (self.0 + 1..).find(|&n| is_prime(n)) {
            self.0 = n;
        Some(self.0)
        } else {
            None
        }
    }
}

#[inline]
fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }

    (2..n).all(|m| n % m != 0)
}

pub struct Problem;

impl Solution for Problem {
    #[inline]
    fn solve() {
        const INPUT: usize = 600851475143;

        println!(
            "{:?}",
            (3..)
                .step_by(2)
                .into_iter()
                .find(|&n| INPUT % n == 0 && is_prime(INPUT / n))
                .map(|n| INPUT / n)
                .unwrap()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::is_prime;
    use super::Prime;

    #[test]
    fn first_10_prime_numbers() {
        assert!(!is_prime(0), "there are only prime numbers greater than 1");
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

        let fib: Vec<usize> = Prime::default()
            .take_while(|&n| n <= 29)
            .collect::<Vec<usize>>();

        assert_eq!(fib, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn run_out_of_numbers() {
        let prime = Prime(usize::MAX);
        let mut it = prime.into_iter();

        assert_eq!(it.next(), None);
    }
}
