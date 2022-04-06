#![feature(test)]
#![allow(unused)]
use indicatif::{ProgressBar, ProgressStyle};
use itertools::{Itertools, MinMaxResult};
use num::traits::Pow;
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use regex::Regex;
use std::cmp::{max, min};

fn main() {
    let input = include_str!("../../resources/input17.txt");

    let regex: Regex = regex();
    let target = parse_input(input, &regex);

    let maxy = 160;
    let minx = 0;

    let hits = hits(maxy, minx, &target);

    println!(
        "highest y: {:}",
        hits.iter().sorted_by_key(|n| n.1).rev().next().unwrap().1
    );
    println!("{:}", hits.len());
}

fn hits(maxy: i64, minx: i64, target: &Area) -> Vec<(i64, i64)> {
    let total: i64 = (maxy - target.miny) * (target.maxx - minx);
    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {wide_bar} {per_sec} {pos:>}/{len:} {eta}"),
    );
    pb.set_draw_rate(20);

    let hits: Vec<_> = (target.miny..=maxy)
        .rev()
        .par_bridge()
        .flat_map(|y| {
            (minx..=target.maxx)
                .filter_map(|x| {
                    let result = check_velocities(x, y, target);

                    pb.inc(1);

                    if result {
                        Some((x, y))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    pb.finish();

    hits
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

fn check_velocities(x0: i64, y0: i64, target: &Area) -> bool {
    let miny = target.miny as f64;
    let maxy = target.maxy as f64;
    let y0 = y0 as f64;

    let abcd: f64 = (2.0 * y0 + 1.0).powi(2);

    let a = (0.5 * ((abcd - 8.0 * maxy).sqrt() + 2.0 * y0 + 1.0)).ceil() as usize;
    let b = (0.5 * ((abcd - 8.0 * miny).sqrt() + 2.0 * y0 + 1.0)).ceil() as usize;
    let c = (-0.5 * ((abcd - 8.0 * maxy).sqrt() + y0 + 0.5).floor()) as usize;
    let d = (-0.5 * ((abcd - 8.0 * miny).sqrt() + y0 + 0.5).floor()) as usize;

    let (min_timesteps, max_timesteps) = match vec![a, b, c, d].iter().minmax() {
        MinMaxResult::MinMax(min, max) => (min.to_owned(), max.to_owned()),
        MinMaxResult::OneElement(minmax) => (minmax.to_owned(), minmax.to_owned()),
        _ => unreachable!(),
    };

    // Calculation approach
    for timestep in 0..=max_timesteps as i64 {
        // let x = (0..=x0).rev().take(timestep as usize).sum::<i64>();
        let x = sumto(x0) - sumto(max(0, x0 - timestep));
        let t = timestep as f64;
        let y = ((y0 as f64 + 0.5) * t - ((t * t) / 2.0)) as i64;
        if (target.minx..=target.maxx).contains(&x) && (target.miny..=target.maxy).contains(&y) {
            return true;
        }
    }

    false
}

#[inline(always)]
fn sumto(n: i64) -> i64 {
    n * (n + 1) / 2
}

#[cfg(test)]
mod test {
    extern crate test;
    use super::*;
    use itertools::MinMaxResult;
    use test::Bencher;

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

        let hit = check_velocities(7, 2, &area);

        assert!(hit);
    }

    fn check_velocities_bench(b: &mut Bencher) {
        let area = Area {
            minx: 20,
            maxx: 30,
            miny: -10,
            maxy: -5,
        };
        b.iter(|| {
            let hit = check_velocities(7, 2, &area);
        });
    }

    #[test]
    fn times_calculation() {
        let timestep = 10;
        for x0 in 0..20 {
            let expected = (0..=x0).rev().take(timestep).sum::<i64>();

            let sumto = |n| n * (n + 1) / 2;
            let actual = sumto(x0) - sumto(max(0, x0 - timestep as i64));
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn position_equal() {
        let x0 = 10;
        let timesteps = 20;

        let expected = {
            let mut x = 0;
            let mut dx = x0;
            for _ in 0..timesteps {
                x += dx;
                dx = max(dx - 1, 0);
            }
            x
        };

        let calculated: i32 = (0..=x0).rev().take(timesteps).sum();

        assert_eq!(expected, calculated);

        let y0 = 10;
        let timesteps = 20;

        print!("expected: ");
        let expected = {
            let mut y = 0;
            let mut dy = y0;
            for _ in 0..timesteps {
                y += dy;
                dy -= 1;
            }
            y
        };

        let t = timesteps as f64;
        let calculated = ((y0 as f64 + 0.5) * t - ((t * t) / 2.0)) as i32;

        println!();
        assert_eq!(expected, calculated as i32);
    }

    #[test]
    fn check_hits() {
        let mut expected = vec![
            (23, -10),
            (25, -9),
            (27, -5),
            (29, -6),
            (22, -6),
            (21, -7),
            (9, 0),
            (27, -7),
            (24, -5),
            (25, -7),
            (26, -6),
            (25, -5),
            (6, 8),
            (11, -2),
            (20, -5),
            (29, -10),
            (6, 3),
            (28, -7),
            (8, 0),
            (30, -6),
            (29, -8),
            (20, -10),
            (6, 7),
            (6, 4),
            (6, 1),
            (14, -4),
            (21, -6),
            (26, -10),
            (7, -1),
            (7, 7),
            (8, -1),
            (21, -9),
            (6, 2),
            (20, -7),
            (30, -10),
            (14, -3),
            (20, -8),
            (13, -2),
            (7, 3),
            (28, -8),
            (29, -9),
            (15, -3),
            (22, -5),
            (26, -8),
            (25, -8),
            (25, -6),
            (15, -4),
            (9, -2),
            (15, -2),
            (12, -2),
            (28, -9),
            (12, -3),
            (24, -6),
            (23, -7),
            (25, -10),
            (7, 8),
            (11, -3),
            (26, -7),
            (7, 1),
            (23, -9),
            (6, 0),
            (22, -10),
            (27, -6),
            (8, 1),
            (22, -8),
            (13, -4),
            (7, 6),
            (28, -6),
            (11, -4),
            (12, -4),
            (26, -9),
            (7, 4),
            (24, -10),
            (23, -8),
            (30, -8),
            (7, 0),
            (9, -1),
            (10, -1),
            (26, -5),
            (22, -9),
            (6, 5),
            (7, 5),
            (23, -6),
            (28, -10),
            (10, -2),
            (11, -1),
            (20, -9),
            (14, -2),
            (29, -7),
            (13, -3),
            (23, -5),
            (24, -8),
            (27, -9),
            (30, -7),
            (28, -5),
            (21, -10),
            (7, 9),
            (6, 6),
            (21, -5),
            (27, -10),
            (7, 2),
            (30, -9),
            (21, -8),
            (22, -7),
            (24, -9),
            (20, -6),
            (6, 9),
            (29, -5),
            (8, -2),
            (27, -8),
            (30, -5),
            (24, -7),
        ];
        expected.sort_unstable();

        let input = "target area: x=20..30, y=-10..-5";
        let regex: Regex = regex();
        let target = parse_input(input, &regex);

        let n = 30;
        let minx = 0;

        assert!(check_velocities(30, -10, &target));

        let mut actual = hits(n, minx, &target);
        actual.sort_unstable();

        assert_eq!(expected, actual);
    }
}
