use std::cmp::{max, min};

use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = include_str!("../../resources/input05.txt");

    let line_regex = Regex::new(r"(\d*),(\d*) -> (\d*),(\d*)").unwrap();

    let lines_tuples = line_regex
        .captures_iter(input)
        .map(|c| {
            (
                c[1].parse::<usize>().unwrap(),
                c[2].parse::<usize>().unwrap(),
                c[3].parse::<usize>().unwrap(),
                c[4].parse::<usize>().unwrap(),
            )
        })
        .collect_vec();

    let maxx = lines_tuples
        .iter()
        .map(|(a, _, b, _)| max(a, b))
        .max()
        .unwrap()
        + 1;
    let maxy = lines_tuples
        .iter()
        .map(|(_, a, _, b)| max(a, b))
        .max()
        .unwrap()
        + 1;
    let size = max(maxx, maxy);
    let mut grid: Vec<usize> = vec![0; size * size];

    for (x1, y1, x2, y2) in lines_tuples {
        if x1 == x2 {
            for i in min(y1, y2)..=max(y1, y2) {
                grid[x1 * size + i] += 1;
            }
        } else if y1 == y2 {
            for i in min(x1, x2)..=max(x1, x2) {
                grid[i * size + y1] += 1;
            }
        } else {
            // Skip non straight lines
        }
    }

    let over2: usize = grid.iter().filter(|&i| *i >= 2).count();

    println!("{}", over2)
}
