use crate::Solution;

pub struct Fibonacci {
    current: usize,
    next: usize,
}

impl Iterator for Fibonacci {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.current + self.next;
        self.current = self.next;
        self.next = new_next;

        Some(self.current)
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self {
            current: 1,
            next: 1,
        }
    }
}

pub struct Problem;

impl Solution for Problem {
    #[inline]
    fn solve() {
        println!(
            "{}",
            Fibonacci::default()
                .take_while(|&n| n <= 4_000_000)
                .filter(|&n| n % 2 == 0)
                .sum::<usize>()
        );
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn first_10_fibonacci_numbers() {
        use super::Fibonacci;

        assert_eq!(
            Fibonacci::default().take(10).collect::<Vec<usize>>(),
            vec![1usize, 2, 3, 5, 8, 13, 21, 34, 55, 89]
        );
    }
}
