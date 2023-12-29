static ADJ: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
static SYMBOLS: [char; 10] = ['#', '$', '%', '&', '*', '+', '-', '/', '=', '@'];

pub fn process(input: &str) -> anyhow::Result<String> {
    let grid: Vec<(usize, Vec<(usize, char)>)> = input
        .lines()
        .map(|l| l.chars().enumerate().collect())
        .enumerate()
        .collect();
    let mut digits: Vec<u32> = vec![];
    for (y, line) in &grid {
        let mut current_number = 0;
        let mut is_number_adj = false;
        for (x, char) in line {
            if char.is_ascii_digit() {
                current_number = current_number * 10 + char.to_digit(10).unwrap_or_default();
                if !is_number_adj {
                    // check adj for this digit
                    is_number_adj = ADJ
                        .iter()
                        .filter_map(|(xx, yy)| {
                            let xx = *x as isize + xx;
                            let yy = *y as isize + yy;
                            if xx > 0 && yy > 0 {
                                return grid
                                    .get(yy as usize)
                                    .map(|(_, row)| row.get(xx as usize).map(|(_, c)| c))
                                    .flatten();
                            }
                            None
                        })
                        .any(|&c| SYMBOLS.iter().any(|&s| c == s));
                }
            }
            if *char == '.' || *x + 1 == line.len() || SYMBOLS.iter().any(|s| s == char) {
                if current_number > 0 {
                    if is_number_adj {
                        digits.push(current_number);
                    }
                    is_number_adj = false;
                    current_number = 0;
                }
            }
        }
    }
    Ok(digits.into_iter().sum::<u32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let input = include_str!("../example.txt");
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
