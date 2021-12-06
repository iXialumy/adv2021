use std::cmp::{max, min};

use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = include_str!("../../resources/input05.txt");

    let over2 = solutionb(input);

    println!("{}", over2)
}

fn solutionb(input: &str) -> usize {
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
            // Assume diagonal lines are 45 degrees or explode
            let ascendingx = x1 < x2;
            let ascendingy = y1 < y2;

            let mut x = x1;
            let mut y = y1;

            grid[x * size + y] += 1;
            for _ in 0..(if ascendingx { x2 - x1 } else { x1 - x2 }) {
                if ascendingx {
                    x += 1
                } else {
                    x -= 1
                };
                if ascendingy {
                    y += 1
                } else {
                    y -= 1
                };
                grid[x * size + y] += 1;
            }
            assert_eq!(x, x2);
            assert_eq!(y, y2);
        }
    }

    let over2: usize = grid.iter().filter(|&i| *i >= 2).count();
    over2
}

#[cfg(test)]
mod test {
    use crate::solutionb;

    #[test]
    fn simple() {
        // ..1.....
        // .221....
        // ..2.....
        // ...1...1
        // ....1.1.
        // .....2..
        // ....1.1.
        // ...1...1

        let input = r"
            2,0 -> 2,2
            1,1 -> 3,1
            7,3 -> 3,7
            1,1 -> 7,7
        ";

        let n = solutionb(input);
        assert_eq!(n, 4);
    }

    #[test]
    fn full() {
        let input = include_str!("../../resources/input05.txt");

        let n = solutionb(input);
        assert_eq!(n, 19374);
    }
}
