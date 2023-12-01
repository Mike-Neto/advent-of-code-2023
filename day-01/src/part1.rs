pub fn process(input: &str) -> anyhow::Result<usize> {
    Ok(input
        .lines()
        .map(|line| {
            let mut it = line.chars();
            let first = it.find(char::is_ascii_digit).unwrap_or_default();
            let last = it.rev().find(char::is_ascii_digit).unwrap_or(first);
            format!("{first}{last}")
                .parse::<usize>()
                .expect("Checked ascii digits failed to parse to usize")
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let input = include_str!("../example.txt");
        assert_eq!(142, process(input)?);
        Ok(())
    }
}
