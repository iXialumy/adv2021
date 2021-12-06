fn main() {
    let input = include_str!("../../resources/input01.txt")
        .lines()
        .filter_map(|line| line.parse::<i32>().ok());

    let mut shifted = input.clone();
    shifted.next();
    let increases = input
        .zip(shifted)
        .filter(|(first, second)| first < second)
        .count();

    println!("Increases: {}", increases);
}
