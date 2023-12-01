pub fn process(input: &str) -> anyhow::Result<String> {
    Ok(input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<char>>()
        })
        .filter_map(|numbers| {
            if let (Some(first), Some(last)) = (numbers.first(), numbers.last()) {
                Some(
                    format!("{first}{last}")
                        .parse::<usize>()
                        .expect("Checked ascii digits failed to parse to usize"),
                )
            } else {
                None
            }
        })
        .sum::<usize>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let input = include_str!("../example.txt");
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
