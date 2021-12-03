use std::str::FromStr;

fn main() {
    let (depth, horizontal, _): (i32, i32, i32) = include_str!("../../resources/input02.txt")
        .lines()
        .filter_map(|line| line.parse::<DirectionLine>().ok())
        .fold(
            (0, 0, 0),
            |(mut depth, mut horizontal, mut aim), DirectionLine(direction, n)| {
                match direction {
                    Direction::Forward => {
                        horizontal += n;
                        depth += aim * n;
                    }
                    Direction::Down => aim += n,
                    Direction::Up => aim -= n,
                }
                (depth, horizontal, aim)
            },
        );

    let total = depth * horizontal;

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
