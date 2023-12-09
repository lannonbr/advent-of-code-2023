use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take_till},
    character::complete::alpha1,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
pub enum Dir {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Network {
    pub instructions: Vec<Dir>,
    pub map: HashMap<String, Node>,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub id: String,
    pub left: String,
    pub right: String,
}

fn parse_input(input: &str) -> IResult<&str, Network> {
    let (input, instructions) = take_till(|c| c == '\n')(input)?;

    let ins = instructions
        .chars()
        .map(|c| if c == 'L' { Dir::Left } else { Dir::Right })
        .collect::<Vec<Dir>>();

    let (input, _) = tag("\n\n")(input)?;

    let (input, nodes) = separated_list1(tag("\n"), parse_line)(input)?;

    Ok((
        input,
        Network {
            instructions: ins,
            map: nodes
                .iter()
                .map(|node| (node.id.clone(), node.clone()))
                .collect(),
        },
    ))
}

fn parse_line(input: &str) -> IResult<&str, Node> {
    let (input, (node_id, _, left, _, right, _)) =
        tuple((alpha1, tag(" = ("), alpha1, tag(", "), alpha1, tag(")")))(input)?;
    Ok((
        input,
        Node {
            id: node_id.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        },
    ))
}

fn run_instructions(start: Node, network: &Network) -> u64 {
    let mut steps_taken = 0;

    let mut curr_location = &start;

    for dir in network.instructions.iter().cycle() {
        match dir {
            Dir::Left => {
                curr_location = network.map.get(&curr_location.left).unwrap();
            }
            Dir::Right => {
                curr_location = network.map.get(&curr_location.right).unwrap();
            }
        }
        steps_taken += 1;

        if curr_location.id.ends_with("Z") {
            return steps_taken;
        }
    }

    0
}

fn main() {
    let input = include_str!("input.txt");

    let (_input, network) = parse_input(input).unwrap();

    let locations: Vec<Node> = network
        .map
        .values()
        .filter(|node| node.id.ends_with("A"))
        .map(|n| n.clone())
        .collect();

    let mut answers: Vec<u64> = Vec::new();

    for node in locations {
        answers.push(run_instructions(node, &network));
    }

    println!("Go calculate the LCM of {:?}", answers);
}
