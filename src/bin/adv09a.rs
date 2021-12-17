use itertools::Itertools;

fn main() {
    // let input = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";
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

    let mut acc = 0;

    for i in 0..x {
        for j in 0..y {
            if is_lower_than_neighbours(&grid, i, j) {
                // dbg!(i, j, grid[i][j]);
                acc += 1 + grid[i][j];
            }
        }
    }

    dbg!(acc);
}

fn is_lower_than_neighbours(grid: &[Vec<i32>], i: usize, j: usize) -> bool {
    let n = grid[i][j];

    let left = j == 0 || n < grid[i][j - 1];
    let up = i == 0 || n < grid[i - 1][j];
    let right = j >= grid[0].len() - 1 || n < grid[i][j + 1];
    let down = i >= grid.len() - 1 || n < grid[i + 1][j];

    left && up && right && down
}
