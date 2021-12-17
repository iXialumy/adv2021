use itertools::Itertools;

fn main() {
    let input = include_str!("../../resources/input10.txt");

    let scores = input
        .lines()
        .filter_map(process_line)
        .sorted_unstable()
        .collect_vec();

    let score = scores.get(scores.len() / 2).unwrap();

    println!("{:?}", score);
}

fn process_line(line: &str) -> Option<i64> {
    let mut stack = Vec::with_capacity(line.len());

    for c in line.trim().chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return None;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return None;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return None;
                }
            }
            '>' => {
                if stack.pop() != Some('<') {
                    return None;
                }
            }
            _ => panic!("Unexpected character: '{}'", c),
        }
    }

    let sum = stack.iter().rev().fold(0, |mut acc, c| {
        acc *= 5;
        acc += match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("Unexpected character: '{}'", c),
        };
        acc
    });

    Some(sum)
}
