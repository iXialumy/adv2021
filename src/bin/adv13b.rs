use std::cmp::max;

use itertools::Itertools;

fn main() {
    let input = include_str!("../../resources/input13.txt");

    let (mut grid, instructions) = parse_input(input);

    for instruction in instructions {
        grid = fold_grid(grid, &instruction);
    }

    let output = grid
        .iter()
        .map(|line| {
            line.iter()
                .map(|c| if c == &'.' { ' ' } else { '█' })
                .join("")
        })
        .join("\n");

    println!("{}", output);
}

fn fold_grid(grid: Vec<Vec<char>>, instruction: &Instruction) -> Vec<Vec<char>> {
    match instruction {
        Instruction::FoldX(x) => grid
            .iter()
            .map(|line| {
                let (front, back) = line.split_at(*x);
                let mut back = back.to_owned();
                back.reverse();
                merge_line(front, &back)
            })
            .collect_vec(),
        Instruction::FoldY(y) => {
            let (front, back) = grid.split_at(*y);
            front
                .iter()
                .zip(back.iter().rev())
                .map(|(line1, line2)| merge_line(line1, line2))
                .collect_vec()
        }
    }
}

fn merge_line(line1: &[char], line2: &[char]) -> Vec<char> {
    line1
        .iter()
        .zip(line2.iter())
        .map(|(c1, c2)| if c1 == &'#' || c2 == &'#' { '#' } else { '.' })
        .collect_vec()
}

#[derive(Debug)]
enum Instruction {
    FoldX(usize),
    FoldY(usize),
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let (coordinates, instructions) = input.split_once("\n\n").unwrap();
    let coordinates = coordinates.lines().map(parse_coordinate).collect_vec();
    let instructions = instructions.lines().map(parse_instruction).collect_vec();

    let (maxx, maxy) = coordinates
        .iter()
        .fold((0, 0), |acc, (x, y)| (max(acc.0, *x), max(acc.1, *y)));

    let mut grid = vec![vec!['.'; maxx + 1]; maxy + 1];
    for (x, y) in coordinates {
        grid[y][x] = '#';
    }

    (grid, instructions)
}

fn parse_coordinate(line: &str) -> (usize, usize) {
    let (x, y) = line.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

fn parse_instruction(line: &str) -> Instruction {
    let (axis, index) = line.split_once('=').unwrap();
    if axis.ends_with('x') {
        Instruction::FoldX(index.parse().unwrap())
    } else if axis.ends_with('y') {
        Instruction::FoldY(index.parse().unwrap())
    } else {
        panic!("Unknown Folding Instruction: {}", line);
    }
}
