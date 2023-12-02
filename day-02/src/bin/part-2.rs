use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i64, newline},
    multi::{separated_list0, separated_list1},
    IResult,
};

#[derive(Debug)]
pub struct Game {
    pub id: i64,
    pub rounds: Vec<Round>,
}

#[derive(Debug)]
pub struct Round {
    pub red: Option<i64>,
    pub green: Option<i64>,
    pub blue: Option<i64>,
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(newline, parse_line)(input)?;

    Ok((input, games))
}

fn parse_line(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = i64(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, rounds) = separated_list0(tag(";"), parse_round)(input)?;

    Ok((input, Game { id, rounds }))
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    let (input, sections) = separated_list0(tag(","), parse_section)(input)?;

    let mut red = None;
    let mut green = None;
    let mut blue = None;

    for section in sections {
        if section.0 == "red" {
            red = Some(section.1)
        }
        if section.0 == "green" {
            green = Some(section.1)
        }
        if section.0 == "blue" {
            blue = Some(section.1)
        }
    }

    Ok((input, Round { red, green, blue }))
}

fn parse_section(input: &str) -> IResult<&str, (&str, i64)> {
    let (input, _) = tag(" ")(input)?;
    let (input, num) = i64(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, word) = alt((tag("red"), tag("blue"), tag("green")))(input)?;

    Ok((input, (word, num)))
}

fn main() {
    let input = include_str!("input.txt");

    let (_, games) = parse_games(input).unwrap();

    let mut sum_powers: i64 = 0;

    for game in games {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for round in game.rounds {
            if round.red.is_some_and(|r| r > max_red) {
                max_red = round.red.unwrap();
            }
            if round.green.is_some_and(|g| g > max_green) {
                max_green = round.green.unwrap();
            }
            if round.blue.is_some_and(|b| b > max_blue) {
                max_blue = round.blue.unwrap();
            }
        }
        sum_powers += max_red * max_blue * max_green;
    }

    println!("{sum_powers}");
}
