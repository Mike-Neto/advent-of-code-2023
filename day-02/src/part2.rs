use crate::{parse_games, Color};

pub fn process(input: &str) -> anyhow::Result<String> {
    let (_, games) = parse_games(input).map_err(|err| err.to_owned())?;

    Ok(games
        .into_iter()
        .map(|g| {
            let reds = g
                .sets
                .iter()
                .filter_map(|c| {
                    c.into_iter()
                        .find(|c| c.color == Color::Red)
                        .map(|c| c.number)
                })
                .max()
                .unwrap_or_default();
            let greens = g
                .sets
                .iter()
                .filter_map(|c| {
                    c.into_iter()
                        .find(|c| c.color == Color::Green)
                        .map(|c| c.number)
                })
                .max()
                .unwrap_or_default();
            let blues = g
                .sets
                .iter()
                .filter_map(|c| {
                    c.into_iter()
                        .find(|c| c.color == Color::Blue)
                        .map(|c| c.number)
                })
                .max()
                .unwrap_or_default();
            reds * greens * blues
        })
        .sum::<u64>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let input = include_str!("../example.txt");
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}
