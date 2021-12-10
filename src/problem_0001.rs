#[inline]
pub fn multiples_of_3_or_5(max: usize) -> Vec<usize> {
    (1..max)
        .into_iter()
        .filter(|&n| n % 3 == 0 || n % 5 == 0)
        .collect()
}
