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
pub fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }

    (2..n).all(|m| n % m != 0)
}
