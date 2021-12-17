use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let input = include_str!("../../resources/input09.txt");

    let lines = input.lines().collect_vec();
    let x = lines.len();
    let y = lines[0].len();

    let grid = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut low_points = HashSet::new();

    for i in 0..x {
        for j in 0..y {
            if is_lower_than_neighbours(&grid, i, j) {
                low_points.insert((i, j));
            }
        }
    }

    let solution: usize = low_points
        .iter()
        .map(|(i, j)| basin_size(&grid, *i, *j))
        .sorted()
        .rev()
        .take(3)
        .product();

    println!("{}", solution)
}

fn basin_size(grid: &[Vec<i32>], i: usize, j: usize) -> usize {
    let mut points = HashSet::new();

    basin_for_low_point(grid, i, j, &mut points);

    points.len()
}

fn basin_for_low_point(
    grid: &[Vec<i32>],
    i: usize,
    j: usize,
    points: &mut HashSet<(usize, usize)>,
) {
    points.insert((i, j));

    if j > 0 && grid[i][j] < grid[i][j - 1] && grid[i][j - 1] < 9 {
        basin_for_low_point(grid, i, j - 1, points);
    }
    if i > 0 && grid[i][j] < grid[i - 1][j] && grid[i - 1][j] < 9 {
        basin_for_low_point(grid, i - 1, j, points);
    }
    if j < grid[0].len() - 1 && grid[i][j] < grid[i][j + 1] && grid[i][j + 1] < 9 {
        basin_for_low_point(grid, i, j + 1, points);
    }
    if i < grid.len() - 1 && grid[i][j] < grid[i + 1][j] && grid[i + 1][j] < 9 {
        basin_for_low_point(grid, i + 1, j, points);
    }
}

fn is_lower_than_neighbours(grid: &[Vec<i32>], i: usize, j: usize) -> bool {
    let n = grid[i][j];

    let left = j == 0 || n < grid[i][j - 1];
    let up = i == 0 || n < grid[i - 1][j];
    let right = j >= grid[0].len() - 1 || n < grid[i][j + 1];
    let down = i >= grid.len() - 1 || n < grid[i + 1][j];

    left && up && right && down
}
