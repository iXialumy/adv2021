#![allow(unused)]
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use num::traits::Pow;
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use regex::Regex;
use std::cmp::{max, min};

fn main() {
    let input = include_str!("../../resources/input17.txt");

    let regex: Regex = regex();
    let target = parse_input(input, &regex);

    let n = 200;

    let total: i64 = (0..n).map(|x| 3 * x).sum();

    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {wide_bar} {pos:>7}/{len:7} {eta}"),
    );

    let results: Vec<_> = (0..n)
        .par_bridge()
        .flat_map(|y| {
            // (0..(min(target.maxx, y)))
            (13..=15)
                .filter_map(|x| {
                    let result = check_velocities(x, y, &target);

                    pb.inc(1);

                    result
                })
                .collect_vec()
        })
        .collect();
    pb.finish();

    println!("{:?}", results.iter().max());
}

fn regex() -> Regex {
    Regex::new(r"target area: x=(\d*)..(\d*), y=(-?\d*)..(-?\d*)").unwrap()
}

#[derive(Debug, PartialEq, Eq)]
struct Area {
    minx: i64,
    maxx: i64,
    miny: i64,
    maxy: i64,
}

fn parse_input(str: &str, regex: &Regex) -> Area {
    let capture = regex.captures_iter(str).next().unwrap();
    Area {
        minx: capture[1].parse().unwrap(),
        maxx: capture[2].parse().unwrap(),
        miny: capture[3].parse().unwrap(),
        maxy: capture[4].parse().unwrap(),
    }
}

fn check_velocities(x: i64, y: i64, target: &Area) -> Option<i64> {
    let max_timesteps =
        ((y as f64) / 2.0 + ((y as f64) * (y as f64) / 4.0 - target.miny as f64)).ceil() as i64;

    let mut dx = x;
    let mut dy = y;
    let mut x = 0;
    let mut y = 0;

    let mut maxy = 0;

    for _ in 0..max_timesteps {
        x += dx;
        y += dy;
        dx = max(dx - 1, 0);
        dy -= 1;

        maxy = max(maxy, y);

        if x <= target.maxx && x >= target.minx && y <= target.maxy && y >= target.miny {
            return Some(maxy);
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let regex = regex();
        let input = "target area: x=20..30, y=-10..-5";

        let expected = Area {
            minx: 20,
            maxx: 30,
            miny: -10,
            maxy: -5,
        };

        let actual = parse_input(input, &regex);
        assert_eq!(expected, actual);
    }

    #[test]
    fn check_velocities_test() {
        let area = Area {
            minx: 20,
            maxx: 30,
            miny: -10,
            maxy: -5,
        };

        let hit = check_velocities(7, 2, &area).is_some();

        assert!(hit);
    }
}
