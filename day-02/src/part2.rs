use crate::{parse_games, Color};

static COLORS: [Color; 3] = [Color::Red, Color::Green, Color::Blue];

/// # Errors
///
/// Will return `Err` parsing fails
pub fn process(input: &str) -> anyhow::Result<u64> {
    #[allow(clippy::redundant_closure_for_method_calls)]
    let (_, games) = parse_games(input).map_err(|err| err.to_owned())?;

    Ok(games
        .into_iter()
        .map(|g| {
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
                .product::<u64>()
        })
        .sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let input = include_str!("../example.txt");
        assert_eq!(2286, process(input)?);
        Ok(())
    }
}
