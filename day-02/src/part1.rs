use crate::parse_games;

static BAG_RED: u64 = 12;
static BAG_GREEN: u64 = 13;
static BAG_BLUE: u64 = 14;

pub fn process(input: &str) -> anyhow::Result<String> {
    let (_, games) = parse_games(input).unwrap(); // TODO Propagate error.

    Ok(games
        .into_iter()
        .filter_map(|g| {
            if !g.sets.into_iter().any(|s| {
                s.into_iter().any(|c| {
                    // TODO should be a match
                    if c.color == "red" {
                        c.number > BAG_RED
                    } else if c.color == "green" {
                        c.number > BAG_GREEN
                    } else if c.color == "blue" {
                        c.number > BAG_BLUE
                    } else {
                        unreachable!("only 3 colors supported");
                    }
                })
            }) {
                return Some(g.id);
            }
            None
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
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
