use anyhow::Context;
use rayon::{iter::ParallelIterator, str::ParallelString};

static SPELLED: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[must_use]
pub fn process(input: &str) -> usize {
    input
        .par_lines()
        .map(|line| {
            let mut it = line.chars().enumerate().filter_map(|(count, c)| {
                if c.is_ascii_digit() {
                    return Some(c);
                }
                let line_sub_str = line.get(count..).unwrap_or_default();
                if let Some(index) = SPELLED
                    .iter()
                    .position(|&digit| line_sub_str.starts_with(digit))
                {
                    return Some(
                        char::from_digit(u32::try_from(index + 1)
                            .context("should never fail since its a position from SPELLED which has only 9 items")
                            .unwrap_or_default(), 10).context("should never fail since its an index of 9 items plus one to give 1-9").unwrap_or('0')
                    );
                }
                None
            });
            let first = it.next().unwrap_or('0');
            let last = it.last().unwrap_or(first);
            format!("{first}{last}")
                .parse::<usize>()
                .context("Checked ascii digits failed to parse to usize").unwrap_or_default()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../example2.txt");
        assert_eq!(281, process(input));
    }
}
