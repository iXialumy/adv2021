use itertools::join;

fn main() {
    let input = include_str!("../../resources/input03.txt");
    let width = input.lines().next().unwrap().len();
    let half_items = input.lines().count() as i32 / 2;

    let hist = adv2021::histogram(input, width);
    let most = hist
        .iter()
        .map(|i| if i > &half_items { 1 } else { 0 })
        .collect::<Vec<_>>();
    let least = most
        .iter()
        .map(|&i| if i == 1 { 0 } else { 1 })
        .collect::<Vec<_>>();

    let gamma = i32::from_str_radix(&join(most, ""), 2).unwrap();
    let epsilon = i32::from_str_radix(&join(least, ""), 2).unwrap();

    println!("{:?}", gamma * epsilon);
}
