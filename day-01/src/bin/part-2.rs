use regex::Regex;

fn main() {
    let input = include_str!("input.txt");

    let numbers = Regex::new(
        r"(zero)|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)|([0-9])",
    )
    .unwrap();

    let mut sum = 0;
    for line in input.lines() {
        let caps: Vec<&str> = numbers
            .captures_iter(line)
            .map(|f| f.get(0).unwrap().as_str())
            .collect();

        let n1 = *caps.get(0).unwrap();
        let n2 = *caps.get(caps.len() - 1).unwrap();

        let n1_parsed = word_or_num(n1.to_string());
        let n2_parsed = word_or_num(n2.to_string());

        let combined = format!("{}{}", n1_parsed, n2_parsed)
            .parse::<u64>()
            .unwrap();
        sum += combined;
    }
    println!("{}", sum);
}

fn word_or_num(s: String) -> u64 {
    if s == "one" {
        return 1;
    } else if s == "two" {
        return 2;
    } else if s == "three" {
        return 3;
    } else if s == "four" {
        return 4;
    } else if s == "five" {
        return 5;
    } else if s == "six" {
        return 6;
    } else if s == "seven" {
        return 7;
    } else if s == "eight" {
        return 8;
    } else if s == "nine" {
        return 9;
    } else if s == "zero" {
        return 0;
    } else {
        return s.parse::<u64>().unwrap();
    }
}
