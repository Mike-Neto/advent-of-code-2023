use anyhow::Context;
use rayon::{iter::ParallelIterator, str::ParallelString};

#[must_use]
pub fn process(input: &str) -> usize {
    input
        .par_lines()
        .map(|line| {
            let mut it = line.chars();
            let first = it.find(char::is_ascii_digit).unwrap_or('0');
            let last = it.rev().find(char::is_ascii_digit).unwrap_or(first);
            format!("{first}{last}")
                .parse::<usize>()
                .context("Checked ascii digits failed to parse to usize")
                .unwrap_or_default()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../example.txt");
        assert_eq!(142, process(input));
    }
}
