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
