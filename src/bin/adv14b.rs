#![feature(test)]

extern crate test;

use std::collections::HashMap;

use adv2021::{frequencies, merge_hashmaps_sum};
use itertools::MinMaxResult::MinMax;
use itertools::{izip, Itertools};

fn main() {
    let input = include_str!("../../resources/input14.txt");

    let (starting_polymer, rules) = input.split_once("\n\n").unwrap();
    let polymer = starting_polymer.to_string();

    let replacements = build_replacement_map(rules);
    let first = polymer.chars().next().unwrap();
    let last = polymer.chars().last().unwrap();

    let mut map = frequencies(
        polymer
            .chars()
            .tuple_windows()
            .map(|(a, b)| format!("{}{}", a, b)),
    );

    for _ in 0..40 {
        map = map_step(&replacements, map);
    }

    let counts = char_counts_from_tuple_counts(map, first, last);

    if let MinMax(min, max) = counts.values().minmax() {
        println!("b: {}", max - min);
    }
}

fn map_step(
    replacements: &HashMap<String, String>,
    inputs: HashMap<String, usize>,
) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for (pair, count) in inputs {
        let mut polymer = pair.to_string();

        polymer = replacement_step(replacements, polymer);
        let mut frequencies = frequencies(
            polymer
                .chars()
                .tuple_windows()
                .map(|(a, b)| format!("{}{}", a, b)),
        );

        for v in frequencies.values_mut() {
            *v *= count;
        }
        merge_hashmaps_sum(&mut map, frequencies);
    }

    map
}

fn char_counts_from_tuple_counts(
    pair_counts: HashMap<String, usize>,
    first: char,
    last: char,
) -> HashMap<char, usize> {
    let mut counts = HashMap::new();

    for (string, count) in pair_counts {
        for c in string.chars() {
            *counts.entry(c).or_insert(0) += count;
        }
    }

    // Every char except first and last have been counted twice now
    // We fix that here
    for (&key, value) in counts.iter_mut() {
        *value /= 2;
        if key == first || key == last {
            *value += 1;
        }
    }

    counts
}

fn build_replacement_map(rules: &str) -> HashMap<String, String> {
    let mut replacements = HashMap::new();
    for line in rules.lines() {
        let (front, back) = line.split_once(" -> ").unwrap();
        let back = format!("{}{}", front.chars().next().unwrap(), back);
        replacements.insert(front.to_string(), back);
    }
    replacements
}

fn replacement_step(replacements: &HashMap<String, String>, polymer: String) -> String {
    let last_char = polymer.chars().last().unwrap();
    let iter = polymer.chars();
    let skip1 = iter.clone().skip(1);

    let replaced = izip!(iter, skip1)
        .map(|(front, back)| format!("{}{}", front, back))
        .map(|key| replacements.get(&key).unwrap())
        .join("");

    format!("{}{}", replaced, last_char)
}

#[cfg(test)]
mod tests {
    use super::*;

    use test::Bencher;

    #[test]
    fn maps_equal() {
        let input = "NNCB\n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C";

        let (starting_polymer, rules) = input.split_once("\n\n").unwrap();
        let mut polymer = starting_polymer.to_string();

        let replacements = build_replacement_map(rules);

        let mut map2 = frequencies(
            starting_polymer
                .chars()
                .tuple_windows()
                .map(|(a, b)| format!("{}{}", a, b)),
        );
        for _ in 0..10 {
            polymer = replacement_step(&replacements, polymer);
            map2 = map_step(&replacements, map2);
        }
        let map1 = frequencies(
            polymer
                .chars()
                .tuple_windows()
                .map(|(a, b)| format!("{}{}", a, b)),
        );

        assert_eq!(map1, map2);
    }

    #[test]
    fn counts_equal() {
        let input = "NNCB\n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C";

        let (starting_polymer, rules) = input.split_once("\n\n").unwrap();
        let mut polymer = starting_polymer.to_string();
        let first = polymer.chars().next().unwrap();
        let last = polymer.chars().last().unwrap();

        let replacements = build_replacement_map(rules);
        let mut map = frequencies(
            starting_polymer
                .chars()
                .tuple_windows()
                .map(|(a, b)| format!("{}{}", a, b)),
        );

        for _ in 0..10 {
            polymer = replacement_step(&replacements, polymer);
            map = map_step(&replacements, map);
        }

        let counts1 = frequencies(polymer.chars());
        let counts2 = char_counts_from_tuple_counts(map, first, last);
        assert_eq!(counts1, counts2);
    }

    #[bench]
    fn bench_naive_10(b: &mut Bencher) {
        let input = "NNCB\n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C";

        let (starting_polymer, rules) = input.split_once("\n\n").unwrap();
        let replacements = build_replacement_map(rules);

        b.iter(|| {
            let mut polymer = starting_polymer.to_string();
            for _ in 0..10 {
                polymer = replacement_step(&replacements, polymer);
            }
            frequencies(polymer.chars());
        })
    }

    #[bench]
    fn bench_hashmap_10(b: &mut Bencher) {
        let input = "NNCB\n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C";

        let (polymer, rules) = input.split_once("\n\n").unwrap();
        let replacements = build_replacement_map(rules);

        b.iter(|| {
            let first = polymer.chars().next().unwrap();
            let last = polymer.chars().last().unwrap();
            let mut map = frequencies(
                polymer
                    .chars()
                    .tuple_windows()
                    .map(|(a, b)| format!("{}{}", a, b)),
            );

            for _ in 0..10 {
                map = map_step(&replacements, map);
            }
            let _counts = char_counts_from_tuple_counts(map, first, last);
        })
    }
}
