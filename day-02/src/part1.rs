use rayon::{iter::ParallelIterator, str::ParallelString};

use crate::parse_game;

static BAG_RED: u64 = 12;
static BAG_GREEN: u64 = 13;
static BAG_BLUE: u64 = 14;

#[must_use]
pub fn process(input: &str) -> u64 {
    input
        .par_lines()
        .filter_map(|line| match parse_game(line) {
            Ok((_, g)) => {
                if !g.sets.into_iter().any(|s| {
                    s.into_iter().any(|c| match c.color {
                        crate::Color::Red => c.number > BAG_RED,
                        crate::Color::Green => c.number > BAG_GREEN,
                        crate::Color::Blue => c.number > BAG_BLUE,
                    })
                }) {
                    return Some(g.id);
                }
                None
            }
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
        assert_eq!(8, process(input));
    }
}
