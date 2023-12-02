use crate::parse_games;

static BAG_RED: u64 = 12;
static BAG_GREEN: u64 = 13;
static BAG_BLUE: u64 = 14;

/// # Errors
///
/// Will return `Err` parsing fails
pub fn process(input: &str) -> anyhow::Result<u64> {
    #[allow(clippy::redundant_closure_for_method_calls)]
    let (_, games) = parse_games(input).map_err(|err| err.to_owned())?;

    Ok(games
        .into_iter()
        .filter_map(|g| {
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
        })
        .sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let input = include_str!("../example.txt");
        assert_eq!(8, process(input)?);
        Ok(())
    }
}
