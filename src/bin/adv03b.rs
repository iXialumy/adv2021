use itertools::Itertools;
use std::cmp::Ordering;

fn main() {
    let input = include_str!("../../resources/input03.txt");
    let lines = input.lines().collect_vec();

    let oxygen_generator_rating = find_oxygen_generator_rating(&lines);

    let co2_scrubber_rating = find_co2_scrubber_rating(&lines);

    println!("{:?}", oxygen_generator_rating * co2_scrubber_rating);
}

fn find_oxygen_generator_rating(lines: &Vec<&str>) -> i32 {
    let m = |ord| match ord {
        Ordering::Less => 0,
        _ => 1,
    };

    find_for_ord(lines, m)
}

fn find_co2_scrubber_rating(lines: &Vec<&str>) -> i32 {
    let m = |ord| match ord {
        Ordering::Less => 1,
        _ => 0,
    };

    find_for_ord(lines, m)
}

fn find_for_ord<F>(lines: &Vec<&str>, m: F) -> i32
where
    F: Fn(Ordering) -> i32,
{
    let mut lines = lines
        .iter()
        .map(|line| line.chars().map(|c| adv2021::parse_bit(c)).collect_vec())
        .collect_vec();
    let mut index = 0;
    while lines.len() > 1 {
        let freq = freq(&lines, index) as usize;
        let ord = freq.cmp(&(lines.len() - freq));
        let c = m(ord);
        lines.retain(|line| line[index] == c);
        index += 1;
    }
    i32::from_str_radix(&lines[0].iter().join(""), 2).unwrap()
}

fn freq(v: &Vec<Vec<i32>>, index: usize) -> i32 {
    v.iter().fold(0, |mut acc, line| {
        acc += line[index];
        acc
    })
}
