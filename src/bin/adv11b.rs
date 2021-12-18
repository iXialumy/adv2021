use itertools::Itertools;

fn main() {
    let input = include_str!("../../resources/input11.txt");

    let mut grid = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    print!("{}", find_zero_step(&mut grid))
}

fn find_zero_step(grid: &mut [Vec<i32>]) -> usize {
    for i in 1.. {
        simulation_step(grid);
        if all_zeroes(grid) {
            return i;
        }
    }
    panic!("Unreachable code");
}

fn all_zeroes(grid: &[Vec<i32>]) -> bool {
    grid.iter()
        .map(|line| line.iter().all(|x| *x == 0))
        .all(|zeroes| zeroes)
}

fn simulation_step(grid: &mut [Vec<i32>]) {
    let mut dirty = true;

    grid.iter_mut()
        .for_each(|line| line.iter_mut().for_each(|i| *i += 1));

    while dirty {
        dirty = false;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] > 9 {
                    increment_neighbours(grid, i, j);
                    dirty = true;
                }
            }
        }
    }
}

fn increment_neighbours(grid: &mut [Vec<i32>], i: usize, j: usize) {
    let boundsx = grid.len() - 1;
    let boundsy = grid[0].len() - 1;

    let minx = if i == 0 { 0 } else { i - 1 };
    let maxx = if i == boundsx { boundsx } else { i + 1 };
    let miny = if j == 0 { 0 } else { j - 1 };
    let maxy = if j == boundsy { boundsy } else { j + 1 };

    #[allow(clippy::needless_range_loop)]
    for x in minx..=maxx {
        for y in miny..=maxy {
            if (x == i && y == j) || grid[x][y] == 0 {
                continue;
            }
            grid[x][y] += 1;
        }
    }

    grid[i][j] = 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment_neighbours_test() {
        let mut input = vec![vec![1, 1, 1], vec![1, 9, 1], vec![1, 1, 1]];

        let expected = vec![vec![2, 2, 2], vec![2, 0, 2], vec![2, 2, 2]];

        increment_neighbours(&mut input, 1, 1);

        assert_eq!(input, expected);
    }

    #[test]
    fn simulation_step_test1() {
        let mut input = vec![
            vec![1, 1, 1, 1, 2],
            vec![1, 9, 9, 9, 1],
            vec![1, 9, 1, 9, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 1, 1, 1, 1],
        ];
        let expected = vec![
            vec![3, 4, 5, 4, 4],
            vec![4, 0, 0, 0, 4],
            vec![5, 0, 0, 0, 5],
            vec![4, 0, 0, 0, 4],
            vec![3, 4, 5, 4, 3],
        ];

        simulation_step(&mut input);

        assert_eq!(input, expected);
    }

    #[test]
    fn simulation_step_test2() {
        let mut input = vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ];
        let expected = vec![
            vec![6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
            vec![3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
            vec![6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
            vec![7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
            vec![7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
            vec![5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
            vec![3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
            vec![7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
            vec![5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
            vec![6, 3, 9, 4, 8, 6, 2, 6, 3, 7],
        ];

        simulation_step(&mut input);

        assert_eq!(input, expected);
    }

    #[test]
    fn full_simulation() {
        let mut grid = vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ];

        let actual = find_zero_step(&mut grid);

        assert_eq!(actual, 195);
    }
}
