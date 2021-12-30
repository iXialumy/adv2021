use std::collections::HashMap;

use adv2021::frequencies;
use itertools::{izip, Itertools};

use itertools::MinMaxResult::MinMax;

fn main() {
    let input = include_str!("../../resources/input14.txt");

    let (polymer, rules) = input.split_once("\n\n").unwrap();
    let mut polymer = polymer.to_string();

    let replacements = build_replacement_map(rules);
    for _ in 0..10 {
        polymer = replacement_step(&replacements, polymer);
    }

    let freq = frequencies(polymer.chars());
    if let MinMax(min, max) = freq.values().minmax() {
        println!("a: {}", max - min);
    }
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
