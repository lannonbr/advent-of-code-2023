use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, u32},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
pub struct RaceRecords {
    pub times: Vec<u32>,
    pub last_maxes: Vec<u32>,
}

fn parse_input(input: &str) -> IResult<&str, RaceRecords> {
    let (input, _) = tuple((tag("Time:"), multispace1))(input)?;
    let (input, times) = separated_list1(multispace1, u32)(input)?;
    let (input, _) = tuple((tag("\nDistance:"), multispace1))(input)?;
    let (input, last_maxes) = separated_list1(multispace1, u32)(input)?;

    Ok((input, RaceRecords { times, last_maxes }))
}

fn run_races(race_records: &RaceRecords) -> u32 {
    let mut result = 1;

    for idx in 0..race_records.times.len() {
        let race_time = race_records.times[idx];
        let last_max = race_records.last_maxes[idx];

        let mut count = 0;

        for i in 1..=race_time {
            let speed = i;
            let time_left = race_time - i;

            let dist = time_left * speed;

            if dist > last_max {
                count += 1;
            }
        }

        result *= count;
    }

    result
}

fn main() {
    let input = include_str!("input.txt");

    let (_input, race_records) = parse_input(input).unwrap();

    let result = run_races(&race_records);

    println!("Result: {}", result);
}
