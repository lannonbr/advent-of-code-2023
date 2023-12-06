use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, i64},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<(i64, i64)>,
    pub maps: Vec<Vec<MapEntry>>,
}

#[derive(Debug)]
pub struct MapEntry {
    pub source: i64,
    pub dest: i64,
    pub size: i64,
    pub from_range: (i64, i64),
    pub to_range: (i64, i64),
}

fn parse_input(input: &str) -> IResult<&str, Almanac> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds_vec) = separated_list1(tag(" "), i64)(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, maps) = separated_list1(tag("\n\n"), parse_map)(input)?;

    let mut seeds = Vec::new();

    for i in (0..seeds_vec.len()).step_by(2) {
        seeds.push((seeds_vec[i], seeds_vec[i + 1]));
    }

    println!("{:?}", seeds);

    let almanac = Almanac { seeds, maps };

    Ok((input, almanac))
}

fn parse_map(input: &str) -> IResult<&str, Vec<MapEntry>> {
    let (input, _) = tuple((alpha1, tag("-to-"), alpha1, tag(" map:\n")))(input)?;
    let mut map_entries = Vec::new();

    let (input, nums) = separated_list1(tag("\n"), separated_list1(tag(" "), i64))(input)?;
    for n in nums {
        map_entries.push(MapEntry {
            source: n[1],
            dest: n[0],
            size: n[2],
            from_range: (n[1], n[1] + n[2]),
            to_range: (n[0], n[0] + n[2]),
        });
    }
    Ok((input, map_entries))
}

fn run_almanac(almanac: &Almanac) -> i64 {
    let mut locations = Vec::new();

    for (g, seed) in almanac.seeds.iter().enumerate() {
        println!("G: {}", g);
        for curr_seed in seed.0..(seed.0 + seed.1) {
            let mut curr_seed_mapping = curr_seed.clone();

            for (_i, map) in almanac.maps.iter().enumerate() {
                'map_entries: for map_entry in map.iter() {
                    if curr_seed_mapping >= map_entry.from_range.0
                        && curr_seed_mapping < map_entry.from_range.1
                    {
                        let jump = map_entry.to_range.0 - map_entry.from_range.0;
                        curr_seed_mapping += jump;
                        break 'map_entries;
                    }
                }
            }
            locations.push(curr_seed_mapping);
        }
    }

    locations.iter().min().unwrap().clone()
}

fn main() {
    let input = include_str!("input.txt");

    let (_input, almanac) = parse_input(input).unwrap();

    let min_location = run_almanac(&almanac);

    println!("Min Location: {}", min_location);
}
