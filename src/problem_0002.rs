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
