pub fn process(input: &str) -> anyhow::Result<String> {
    Ok(input
        .lines()
        .map(|line| {
            let spelled = [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ];
            let mut results: Vec<usize> = vec![];
            for (index, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    results.push(
                        format!("{c}")
                            .parse::<usize>()
                            .expect("Checked ascii digits failed to parse to usize"),
                    );
                } else {
                    let sub = line.chars().skip(index).collect::<String>();
                    if let Some(digit) = spelled.iter().position(|&digit| sub.starts_with(digit)) {
                        results.push(digit + 1)
                    }
                }
            }
            results
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
        let input = include_str!("../example2.txt");
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
