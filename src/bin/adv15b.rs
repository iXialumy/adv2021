use itertools::Itertools;
use petgraph::algo::astar;
use petgraph::graphmap::{DiGraphMap, GraphMap};

fn main() {
    let input = include_str!("../../resources/input15.txt");
    let grid = to_2d_grid(input);

    let graph = create_graph_from_grid(&grid);
    let start = (0, 0);
    let goal = (grid.len() - 1, grid[0].len() - 1);

    if let Some((cost, _path)) = astar(&graph, start, |node| node == goal, |e| *e.2, |_| 0) {
        print!("{}", cost);
    } else {
        panic!("No shortest path found");
    }
}

fn to_2d_grid(input: &str) -> Vec<Vec<usize>> {
    let base_grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let expanded_line: Vec<Vec<usize>> = base_grid
        .iter()
        .map(|line: &Vec<usize>| {
            (0..5)
                .flat_map(|i| line.iter().map(move |&n| rollover_increment(n, i)))
                .collect_vec()
        })
        .collect_vec();

    let mut expanded_grid = Vec::new();
    for i in 0..5 {
        for line in &expanded_line {
            let incremented_line = line
                .iter()
                .map(move |&n| rollover_increment(n, i))
                .collect_vec();
            expanded_grid.push(incremented_line);
        }
    }

    expanded_grid
}

fn rollover_increment(i: usize, j: usize) -> usize {
    (i - 1 + j) % 9 + 1
}

fn create_graph_from_grid(
    grid: &[Vec<usize>],
) -> GraphMap<(usize, usize), usize, petgraph::Directed> {
    let mut graph = DiGraphMap::new();

    for (i, line) in grid.iter().enumerate() {
        for (j, &v) in line.iter().enumerate() {
            graph.add_node((i, j));
            if i > 0 {
                graph.add_edge((i - 1, j), (i, j), v);
            }

            if j > 0 {
                graph.add_edge((i, j - 1), (i, j), v);
            }

            if i < grid.len() - 1 {
                graph.add_edge((i + 1, j), (i, j), v);
            }

            if j < line.len() - 1 {
                graph.add_edge((i, j + 1), (i, j), v);
            }
        }
    }

    graph
}
