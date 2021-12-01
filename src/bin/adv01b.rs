use itertools::izip;

fn main() {
    let input = include_str!("../../resources/input01.txt")
        .lines()
        .filter_map(|line| line.parse::<i32>().ok());

    let shift1 = input.clone().skip(1);
    let shift2 = input.clone().skip(2);

    let triples = izip!(input, shift1, shift2).map(|(a, b, c)| a + b + c);

    let shifted = triples.clone().skip(1);
    let increases = triples
        .zip(shifted)
        .filter(|(first, second)| first < second)
        .count();

    println!("Increases: {}", increases);
}
