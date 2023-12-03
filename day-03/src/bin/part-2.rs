use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();

        grid.push(chars);
    }

    let nums = parse_nums(&grid);

    let mut hash = HashMap::new();

    let mut gear_sum = Vec::new();

    for (y, line) in grid.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == '*' {
                let adjacents = browse_around(&grid, x, y);

                for adj in adjacents {
                    let num: Num = get_num(adj, &nums).unwrap();
                    hash.insert(num.range.0, num.num);
                }

                if hash.len() == 2 {
                    gear_sum.push(hash.iter().map(|n| n.1).product::<u64>());
                }
                hash.clear();
            }
        }
    }

    println!("{:?}", gear_sum.iter().sum::<u64>());
}

fn browse_around(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut adjacents = Vec::new();
    let yLimit = grid.len();
    let xLimit = grid[0].len();

    // up left
    if x - 1 >= 0 && y - 1 >= 0 && grid[y - 1][x - 1].is_digit(10) {
        adjacents.push((x - 1, y - 1));
    }

    // up
    if y - 1 >= 0 && grid[y - 1][x].is_digit(10) {
        adjacents.push((x, y - 1));
    }

    // up right
    if x + 1 < xLimit && y - 1 >= 0 && grid[y - 1][x + 1].is_digit(10) {
        adjacents.push((x + 1, y - 1));
    }

    // right
    if x + 1 < xLimit && grid[y][x + 1].is_digit(10) {
        adjacents.push((x + 1, y));
    }

    // down right
    if x + 1 < xLimit && y + 1 < yLimit && grid[y + 1][x + 1].is_digit(10) {
        adjacents.push((x + 1, y + 1));
    }

    // down
    if y + 1 < yLimit && grid[y + 1][x].is_digit(10) {
        adjacents.push((x, y + 1));
    }

    // down left
    if x - 1 >= 0 && y + 1 < yLimit && grid[y + 1][x - 1].is_digit(10) {
        adjacents.push((x - 1, y + 1));
    }

    // left
    if x - 1 >= 0 && grid[y][x - 1].is_digit(10) {
        adjacents.push((x - 1, y));
    }

    adjacents
}

fn get_num((x, y): (usize, usize), nums: &Vec<Num>) -> Option<Num> {
    for n in nums.iter() {
        if y == n.range.0.y {
            if x >= n.range.0.x && x <= n.range.1.x {
                return Some(n.clone());
            }
        }
    }

    return None;
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub struct Num {
    pub num: u64,
    pub range: (Point, Point),
}

fn parse_nums(grid: &Vec<Vec<char>>) -> Vec<Num> {
    let mut working_num_str: String = String::new();
    let mut nums = Vec::new();

    let mut start_point = Point { x: 0, y: 0 };

    for (y, line) in grid.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if grid[y][x].is_digit(10) {
                if working_num_str.is_empty() {
                    start_point = Point { x, y };
                }
                working_num_str.push(grid[y][x]);
            } else {
                if working_num_str.is_empty() {
                    continue;
                } else {
                    nums.push(Num {
                        num: working_num_str.parse::<u64>().unwrap(),
                        range: (start_point, Point { x: x - 1, y }),
                    });
                    start_point = Point { x: 0, y: 0 };
                    working_num_str.clear();
                }
            }
        }

        if working_num_str.is_empty() {
            continue;
        } else {
            nums.push(Num {
                num: working_num_str.parse::<u64>().unwrap(),
                range: (
                    start_point,
                    Point {
                        x: line.len() - 1,
                        y,
                    },
                ),
            });
            start_point = Point { x: 0, y: 0 };
            working_num_str.clear();
        }
    }

    nums
}
