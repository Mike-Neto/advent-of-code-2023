pub mod part1;
pub mod part2;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, u64},
    multi::separated_list1,
    sequence::delimited,
};

#[derive(Debug)]
struct Cube<'a> {
    color: &'a str,
    number: u64,
}

#[derive(Debug)]
struct Game<'a> {
    id: u64,
    sets: Vec<Vec<Cube<'a>>>,
}
fn parse_cube(input: &str) -> nom::IResult<&str, Cube> {
    let (input, number) = u64(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = alpha1(input)?;
    Ok((input, Cube { color, number }))
}

fn parse_set(input: &str) -> nom::IResult<&str, Vec<Cube>> {
    separated_list1(tag(", "), parse_cube)(input)
}

fn parse_game(input: &str) -> nom::IResult<&str, Game> {
    let (input, id) = delimited(tag("Game "), u64, tag(": "))(input)?;
    let (input, sets) = separated_list1(tag("; "), parse_set)(input)?;

    Ok((input, Game { id, sets }))
}

fn parse_games(input: &str) -> nom::IResult<&str, Vec<Game>> {
    separated_list1(newline, parse_game)(input)
}
