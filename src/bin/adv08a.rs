use itertools::Itertools;

fn main() {
    let input = include_str!("../../resources/input08.txt")
        .lines()
        .map(|line| line.split_once(" | ").unwrap().1)
        .map(|nums| nums.split(' ').collect_vec())
        .collect_vec();

    let count = input
        .iter()
        .map(|line| {
            line.iter()
                .map(|segment| match segment.len() {
                    2 | 4 | 3 | 7 => 1,
                    _ => 0,
                })
                .sum::<i32>()
        })
        .sum::<i32>();

    println!("{}", count);
}
