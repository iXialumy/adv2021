use itertools::{Itertools, MinMaxResult};

fn main() {
    let input = include_str!("../../resources/input07.txt")
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect_vec();

    let lowest = find_lowest_fuel_cost(&input);

    println!("{}", lowest);
}

fn find_lowest_fuel_cost(input: &[usize]) -> usize {
    let minmax = input.iter().minmax();

    let (min, max) = match minmax {
        MinMaxResult::MinMax(x, y) => {
            (*x,*y)
        }
        MinMaxResult::OneElement(x) => return *x,
        MinMaxResult::NoElements => panic!("Empty input list"),
    };

    let range = max - min;
    let mut costs = vec![0; range];

    for i in min..max {
        costs[i - min] = input
            .iter()
            .map(|x| {
                let n = x.abs_diff(i);
                n * (n + 1) / 2 // Sum of numbers 1..n
            })
            .sum();
    }

    *costs.iter().min().unwrap()
}
