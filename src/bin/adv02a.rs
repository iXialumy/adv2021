use std::{collections::HashMap, str::FromStr};

fn main() {
    let direction_counter: HashMap<Direction, i32> = include_str!("../../resources/input02.txt")
        .lines()
        .filter_map(|line| line.parse::<DirectionLine>().ok())
        .fold(
            HashMap::<Direction, i32>::new(),
            |mut acc, DirectionLine(direction, n)| {
                *acc.entry(direction).or_insert(0) += n;
                acc
            },
        );
    let total_down = direction_counter.get(&Direction::Down).unwrap_or(&0)
        - direction_counter.get(&Direction::Up).unwrap_or(&0);
    let total = total_down * direction_counter.get(&Direction::Forward).unwrap_or(&0);
    println!("{}", total);
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    Forward,
    Down,
    Up,
}

struct DirectionLine(Direction, i32);

impl FromStr for DirectionLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, n) = s.split_once(" ").ok_or(())?;

        let direction = match direction {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            n => panic!("Could not parse direction: {}", n),
        };
        let n = n.parse().unwrap();
        Ok(DirectionLine(direction, n))
    }
}
