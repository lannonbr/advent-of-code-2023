fn produce_diff_sequence(seq: Vec<i64>) -> Vec<i64> {
    let mut diff = Vec::new();

    for i in 0..seq.len() - 1 {
        diff.push(seq[i + 1] - seq[i]);
    }

    diff
}

fn extrapolate(line_seq_list: Vec<Vec<i64>>) -> i64 {
    let mut new_seq_list: Vec<Vec<i64>> = Vec::new();

    for (i, idx) in (0..line_seq_list.len()).rev().enumerate() {
        let mut new_seq: Vec<i64> = line_seq_list[idx].clone();

        if i == 0 {
            new_seq.push(0);
        } else {
            new_seq.push(new_seq_list[i - 1].last().unwrap() + new_seq.last().unwrap());
        }

        new_seq_list.push(new_seq);
    }

    new_seq_list.last().unwrap().last().unwrap().to_owned()
}

fn main() {
    let input = include_str!("input.txt");

    let mut nums: Vec<i64> = Vec::new();

    for line in input.lines() {
        let orig_seq: Vec<i64> = line
            .split_whitespace()
            .map(|c| c.parse::<i64>().unwrap())
            .collect();

        let mut line_seq_list = vec![orig_seq];

        while !line_seq_list.last().unwrap().iter().all(|n| *n == 0) {
            line_seq_list.push(produce_diff_sequence(
                line_seq_list.last().unwrap().to_vec(),
            ));
        }

        let num = extrapolate(line_seq_list);

        nums.push(num);
    }

    println!("SUM: {}", nums.iter().sum::<i64>());
}
