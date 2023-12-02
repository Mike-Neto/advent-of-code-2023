pub mod part1;
pub mod part2;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, u64},
    multi::separated_list1,
    sequence::delimited,
};

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Cube {
    color: Color,
    number: u64,
}

#[derive(Debug)]
struct Game {
    id: u64,
    sets: Vec<Vec<Cube>>,
}

fn parse_cube(input: &str) -> nom::IResult<&str, Cube> {
    let (input, number) = u64(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = alpha1(input).map(|(input, c)| {
        let color = match c {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => unreachable!("only 3 colors supported"),
        };
        (input, color)
    })?;
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
