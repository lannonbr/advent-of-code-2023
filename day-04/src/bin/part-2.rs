use std::collections::{BTreeMap, VecDeque};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
pub struct Scratchcard {
    pub card_id: usize,
    pub winning_numbers: Vec<usize>,
    pub my_numbers: Vec<usize>,
}

impl Scratchcard {
    fn get_wins(&self) -> usize {
        let mut wins = 0;
        for num in self.my_numbers.iter() {
            if self.winning_numbers.contains(num) {
                wins += 1;
            }
        }

        wins
    }
}

fn main() {
    let input = include_str!("input.txt");

    let (_, cards) = parse_cards(input).unwrap();

    let card_map: BTreeMap<usize, Scratchcard> =
        cards.into_iter().map(|card| (card.card_id, card)).collect();

    let mut cards_to_process: VecDeque<usize> = card_map.keys().cloned().collect();

    let mut cards_processed = 0;

    while cards_to_process.len() > 0 {
        let id = cards_to_process.pop_front().unwrap();
        let card = card_map.get(&id).unwrap();
        let wins = card.get_wins();
        for i in 1..wins + 1 {
            cards_to_process.push_back(id + i);
        }
        cards_processed += 1;
    }

    println!("{}", cards_processed);
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Scratchcard>> {
    let (input, cards) = separated_list1(newline, parse_line)(input)?;

    Ok((input, cards))
}

fn parse_line(input: &str) -> IResult<&str, Scratchcard> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, id) = digit1(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, (winning, my)) = separated_pair(
        separated_list1(multispace1, digit1),
        separated_pair(multispace1, tag("|"), multispace1),
        separated_list1(multispace1, digit1),
    )(input)?;

    let card = Scratchcard {
        card_id: id.parse::<usize>().unwrap(),
        winning_numbers: winning
            .iter()
            .map(|n| n.parse::<usize>().unwrap())
            .collect(),
        my_numbers: my.iter().map(|n| n.parse::<usize>().unwrap()).collect(),
    };

    Ok((input, card))
}
