use std::collections::HashSet;

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

pub fn process(input: &str) -> anyhow::Result<String> {
    let grid: Vec<(usize, Vec<(usize, char)>)> = input
        .lines()
        .map(|l| l.chars().enumerate().collect())
        .enumerate()
        .collect();

    let mut gears: Vec<usize> = vec![];
    for (y, line) in &grid {
        for (x, c) in line {
            if *c == '*' {
                let adj_numbers: HashSet<usize> = ADJ
                    .iter()
                    .filter_map(|(xx, yy)| {
                        let xx = *x as isize + xx;
                        let yy = *y as isize + yy;

                        let el = grid
                            .get(yy as usize)
                            .map(|(_, row)| row.get(xx as usize).map(|(_, c)| *c))
                            .flatten();
                        if let Some(el) = el {
                            if el.is_ascii_digit() {
                                // found a digit, now find a way to get all horizontally sequencially adjacendaty digits.
                                let mut left_digits: Vec<char> = vec![];
                                let mut right_digits: Vec<char> = vec![];
                                // start by going left
                                let mut count = 1;
                                loop {
                                    if let Some(d) = grid
                                        .get(yy as usize)
                                        .map(|(_, row)| {
                                            let k = xx as isize - count;
                                            if k >= 0 {
                                                return row.get(k as usize).map(|(_, c)| *c);
                                            }
                                            None
                                        })
                                        .flatten()
                                    {
                                        if d.is_ascii_digit() {
                                            left_digits.push(d);
                                            count += 1;
                                        } else {
                                            break;
                                        }
                                    } else {
                                        break;
                                    }
                                }

                                left_digits.reverse();
                                left_digits.push(el);
                                count = 1;

                                loop {
                                    if let Some(d) = grid
                                        .get(yy as usize)
                                        .map(|(_, row)| {
                                            row.get((xx + count) as usize).map(|(_, c)| *c)
                                        })
                                        .flatten()
                                    {
                                        if d.is_ascii_digit() {
                                            right_digits.push(d);
                                            count += 1;
                                        } else {
                                            break;
                                        }
                                    } else {
                                        break;
                                    }
                                }

                                let digit = vec![left_digits, right_digits]
                                    .concat()
                                    .into_iter()
                                    .rev()
                                    .enumerate()
                                    .map(|(i, c)| {
                                        c.to_digit(10)
                                            .map(|n| n as usize * 10usize.pow(i as u32))
                                            .expect("failed to conver to digist")
                                    })
                                    .sum::<usize>();

                                Some(digit)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect();

                if adj_numbers.len() == 2 {
                    gears.push(adj_numbers.into_iter().product())
                }
            }
        }
    }

    Ok(gears.into_iter().sum::<usize>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let input = include_str!("../example.txt");
        assert_eq!("467835", process(input)?);
        Ok(())
    }
}
