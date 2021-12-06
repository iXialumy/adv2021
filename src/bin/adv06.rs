const SIZE: usize = 9;

fn main() {
    let input = include_str!("../../resources/input06.txt");

    let mut nums: [u128; SIZE] = input.split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .fold([0u128; SIZE], |mut acc, n|  {
            acc[n as usize] += 1;
            acc
        });

    for _ in 0..80 {
        simulate_day(&mut nums);
    }

    println!("After 80 days: {}", nums.iter().sum::<u128>());

    for _ in 0..(256-80) {
        simulate_day(&mut nums);
    }

    println!("After 256 days: {}", nums.iter().sum::<u128>());
}

fn simulate_day(nums: &mut [u128; SIZE]) {
    let mut next = [0u128; SIZE];
    next[..(SIZE - 1)].clone_from_slice(&nums[1..SIZE]);

    next[6] += nums[0];
    next[8] = nums[0];

    nums.clone_from_slice(&next);
}