static SPELLED: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn process(input: &str) -> anyhow::Result<usize> {
    Ok(input
        .lines()
        .map(|line| {
            let mut it = line.chars().enumerate().filter_map(|(count, c)| {
                if c.is_ascii_digit() {
                    return Some(c);
                } else {
                    let line_sub_str = line.get(count..).unwrap_or_default();
                    if let Some(index) = SPELLED
                        .iter()
                        .position(|&digit| line_sub_str.starts_with(digit))
                    {
                        return Some((index + 49) as u8 as char); // like wft, but + 48 for ascii offset + 1 for index offset
                    } else {
                        return None;
                    }
                }
            });
            let first = it.next().unwrap_or('0');
            let last = it.last().unwrap_or(first);
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
        let input = include_str!("../example2.txt");
        assert_eq!(281, process(input)?);
        Ok(())
    }
}
