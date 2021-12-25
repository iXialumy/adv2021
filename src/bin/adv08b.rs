use std::collections::HashMap;

use itertools::Itertools;
use num::pow;

const NUMBERS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

fn main() {
    let input = include_str!("../../resources/input08.txt")
        .lines()
        .map(|line| line.trim().split_once(" | ").unwrap())
        .map(|(front, back)| {
            (
                front.split(' ').collect_vec(),
                back.split(' ').collect_vec(),
            )
        })
        .collect_vec();

    let count = input
        .iter()
        .map(|(front, back)| handle_line(front, back))
        .sum::<usize>();

    println!("{}", count);
}

fn handle_line(front: &[&str], back: &[&str]) -> usize {
    let mappings = find_mappings(front);

    back.iter()
        .map(|digit| rewire(digit, &mappings))
        .map(get_digit)
        .rev()
        .enumerate()
        .map(|(index, digit)| digit * pow(10, index))
        .sum()
}

fn rewire<'a>(digit: &&str, mappings: &'a HashMap<char, char>) -> Vec<&'a char> {
    digit
        .chars()
        .map(|c| match mappings.get(&c) {
            Some(x) => x,
            None => panic!("No match for {} found", c),
        })
        .collect_vec()
}

/// Convert set of segments to corresponding digit
fn get_digit(chars: Vec<&char>) -> usize {
    let sorted_chars = chars.iter().sorted_unstable().join("");
    for (i, &num) in NUMBERS.iter().enumerate() {
        if sorted_chars == num {
            return i;
        }
    }
    panic!("Not a valid number: {}", sorted_chars)
}

/// Find wiring mappings for segments
fn find_mappings(encoded_numbers: &[&str]) -> HashMap<char, char> {
    let default = "abcdefg";

    let mapping = default
        .chars()
        .permutations(default.len())
        .map(|combination| {
            combination
                .iter()
                .cloned()
                .zip(default.chars())
                .collect::<HashMap<char, char>>()
        })
        .find(|combination| check_combination(combination, encoded_numbers));

    match mapping {
        Some(m) => m,
        None => panic!(
            "Could not find any valid mapping for: {:?}",
            encoded_numbers
        ),
    }
}

/// Check if a given mapping is a correct mapping for numbers
fn check_combination(mapping: &HashMap<char, char>, numbers: &[&str]) -> bool {
    let mut sorted = NUMBERS;
    sorted.sort_unstable();

    let mapped = numbers
        .iter()
        .map(|number| rewire(number, mapping))
        .map(|mut num| {
            num.sort();
            num
        })
        .map(|num| num.iter().join(""))
        .sorted_unstable()
        .collect_vec();

    sorted.iter().zip(mapped.iter()).all(|(s, m)| s == m)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use itertools::Itertools;

    use crate::{check_combination, handle_line};

    #[test]
    fn line1() {
        let line =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let (front, back) = line.split_once(" | ").unwrap();

        let front = front.split(' ').collect_vec();
        let back = back.split(' ').collect_vec();
        assert_eq!(handle_line(&front, &back), 5353);
    }

    #[test]
    fn check_combination_test() {
        let numbers = [
            "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab",
        ];
        let mut mapping = HashMap::new();
        mapping.insert('d', 'a');
        mapping.insert('e', 'b');
        mapping.insert('a', 'c');
        mapping.insert('f', 'd');
        mapping.insert('g', 'e');
        mapping.insert('b', 'f');
        mapping.insert('c', 'g');

        assert!(check_combination(&mapping, &numbers));
    }
}
