use std::collections::HashMap;
use std::hash::Hash;

pub fn histogram(input: &str, width: usize) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.chars().map(parse_bit))
        .fold(vec![0; width], |mut acc, line| {
            for (i, n) in line.enumerate() {
                acc[i] += n;
            }
            acc
        })
}

pub fn parse_bit(c: char) -> i32 {
    if c == '0' {
        0
    } else {
        1
    }
}

pub fn frequencies<T: Eq + Hash, I>(iterable: I) -> HashMap<T, usize>
where
    I: Iterator<Item = T>,
{
    let mut map = HashMap::new();

    for elem in iterable {
        let counter = map.entry(elem).or_insert(0);
        *counter += 1;
    }

    map
}

pub fn merge_hashmaps_sum<T: Eq + Hash>(acc: &mut HashMap<T, usize>, map: HashMap<T, usize>) {
    for (k, v) in map {
        *acc.entry(k).or_insert(0) += v;
    }
}
