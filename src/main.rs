fn day_01_part_01(path: &str) -> Result<usize, std::io::Error> {
    let data = std::fs::read_to_string(path)?;
    Ok(data
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
        .sum())
}

fn day_01_part_02(path: &str) -> Result<usize, std::io::Error> {
    let data = std::fs::read_to_string(path)?;
    Ok(data
        .lines()
        .map(|line| {
            let spelled = vec![
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
        .sum())
}

fn main() -> Result<(), std::io::Error> {
    let result = day_01_part_01("inputs/day_01/data.txt")?;
    println!("{result} is the sum of the number made out of the first and last digit of each line");
    let result = day_01_part_02("inputs/day_01/data.txt")?;
    println!("{result} is the sum of the number made out of the first and last plain or spelled digit of each line");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{day_01_part_01, day_01_part_02};

    #[test]
    fn day_01_part_01_works() {
        let result = day_01_part_01("inputs/day_01/example.txt").unwrap();
        assert_eq!(result, 142);
    }

    #[test]
    fn day_01_part_02_works() {
        let result = day_01_part_02("inputs/day_01/example_part_2.txt").unwrap();
        assert_eq!(result, 281);
    }
}
