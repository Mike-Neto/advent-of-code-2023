use rayon::{iter::ParallelIterator, str::ParallelString};

use crate::{parse_game, Color};

static COLORS: [Color; 3] = [Color::Red, Color::Green, Color::Blue];

#[must_use]
pub fn process(input: &str) -> u64 {
    input
        .par_lines()
        .filter_map(|line| match parse_game(line) {
            Ok((_, g)) => Some(
                COLORS
                    .iter()
                    .filter_map(|color| {
                        g.sets
                            .iter()
                            .map(|set| {
                                set.iter()
                                    .filter_map(|c| {
                                        if c.color == *color {
                                            Some(c.number)
                                        } else {
                                            None
                                        }
                                    })
                                    .max()
                            })
                            .max()
                            .unwrap_or_default()
                    })
                    .product::<u64>(),
            ),
            Err(err) => {
                println!("{err}");
                None
            }
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../example.txt");
        assert_eq!(2286, process(input));
    }
}
