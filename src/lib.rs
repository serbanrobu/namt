pub type Number = u128;

#[inline]
pub fn find_unsafe_number(safe: usize, numbers: &[Number]) -> Option<Number> {
    numbers
        .iter()
        .skip(safe)
        .zip(numbers.windows(safe))
        .find_map(|(&n, prev)| {
            if prev
                .iter()
                .enumerate()
                .flat_map(|(i, &m)| prev[i + 1..].iter().map(move |&n| m + n))
                .any(|s| s == n)
            {
                return None;
            }

            Some(n)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let numbers: Vec<Number> = EXAMPLE
            .lines()
            .map(|line| line.parse().expect("should parse number"))
            .collect();

        let unsafe_number =
            find_unsafe_number(5, &numbers).expect("should find an unsecured number");

        assert_eq!(unsafe_number, 127);
    }

    const EXAMPLE: &str = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
";
}
