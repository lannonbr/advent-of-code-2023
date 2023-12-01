use regex::Regex;

fn main() {
    let input = include_str!("input.txt");

    let get_rid_of_letters_regex = Regex::new(r"[a-z]").unwrap();

    let mut sum = 0;
    for line in input.lines() {
        let nums_str = get_rid_of_letters_regex.replace_all(line, "");
        let n1 = nums_str.chars().nth(0).unwrap();
        let n2 = nums_str.chars().nth(nums_str.len() - 1).unwrap();
        let combined = format!("{}{}", n1, n2).parse::<u64>().unwrap();

        sum += combined;
    }
    println!("{}", sum);
}
