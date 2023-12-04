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
    fn get_score(&self) -> usize {
        let mut score = 0;
        for num in self.my_numbers.iter() {
            if self.winning_numbers.contains(num) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }

        score
    }
}

fn main() {
    let input = include_str!("input.txt");

    let (_, cards) = parse_cards(input).unwrap();

    let sum: usize = cards.iter().map(|c| c.get_score()).sum();

    println!("Sum: {}", sum);
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
