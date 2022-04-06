use std::collections::HashSet;

use itertools::Itertools;
use petgraph::graphmap::{GraphMap, UnGraphMap};
use petgraph::Undirected;

fn main() {
    let input = include_str!("../../resources/input12.txt");

    let graph = parse_input(input);
    let paths = paths_with_max_length2(&graph, vec!["start"]);

    println!("{}", paths.len());
}

fn parse_input(input: &str) -> GraphMap<&str, (), Undirected> {
    println!("Parsing inputs...");
    let tuples = input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect_vec();

    UnGraphMap::<&str, ()>::from_edges(tuples)
}

fn paths_with_max_length2<'a>(
    graph: &GraphMap<&'a str, (), Undirected>,
    path: Vec<&'a str>,
) -> Vec<Vec<&'a str>> {
    let mut out = Vec::new();

    let node = path.last().unwrap();

    for n in graph.neighbors(node) {
        if n == "start" {
            continue;
        }

        if n.chars().all(|c| c.is_lowercase())
            && path.contains(&n)
            && has_duplicate_small_caves(&path)
        {
            continue;
        }

        let mut cloned = path.to_owned();
        cloned.push(n);

        if n == "end" {
            out.push(cloned);
            continue;
        }

        out.append(&mut paths_with_max_length2(graph, cloned));
    }

    out
}

fn has_duplicate_small_caves(list: &[&str]) -> bool {
    let mut set = HashSet::new();
    for s in list {
        if !s.chars().all(|c| c.is_lowercase()) {
            continue;
        }
        if set.contains(s) {
            return true;
        } else {
            set.insert(s);
        }
    }

    false
}
