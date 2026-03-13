/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .enumerate()
        .try_fold((0, 0), |(s, n), (i, c)| {
            let mut d = c.to_digit(10)?;
            if i & 1 == 1 {
                d = d * 2 - if d > 4 { 9 } else { 0 };
            }
            Some((s + d, n + 1))
        })
        .is_some_and(|(s, n)| n > 1 && s % 10 == 0)
}
