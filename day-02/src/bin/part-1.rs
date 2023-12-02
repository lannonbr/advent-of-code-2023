use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i16, newline},
    multi::{separated_list0, separated_list1},
    IResult,
};

#[derive(Debug)]
pub struct Game {
    pub id: i16,
    pub rounds: Vec<Round>,
}

#[derive(Debug)]
pub struct Round {
    pub red: Option<i16>,
    pub green: Option<i16>,
    pub blue: Option<i16>,
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(newline, parse_line)(input)?;

    Ok((input, games))
}

fn parse_line(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = i16(input)?;
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

fn parse_section(input: &str) -> IResult<&str, (&str, i16)> {
    let (input, _) = tag(" ")(input)?;
    let (input, num) = i16(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, word) = alt((tag("red"), tag("blue"), tag("green")))(input)?;

    Ok((input, (word, num)))
}

fn main() {
    let input = include_str!("input.txt");

    let (_, games) = parse_games(input).unwrap();

    let mut sum_ids = 0;

    let limit_red = 12;
    let limit_green = 13;
    let limit_blue = 14;

    for game in games {
        let mut possible_game = true;
        for round in game.rounds {
            if round.red.is_some_and(|r| r > limit_red) {
                possible_game = false
            }
            if round.green.is_some_and(|g| g > limit_green) {
                possible_game = false
            }
            if round.blue.is_some_and(|b| b > limit_blue) {
                possible_game = false
            }
        }
        if possible_game {
            sum_ids += game.id
        }
    }

    println!("{sum_ids}");
}
