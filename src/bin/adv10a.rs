fn main() {
    let input = include_str!("../../resources/input10.txt");

    let score: i32 = input.lines().filter_map(process_line).sum();

    println!("{}", score);
}

fn process_line(line: &str) -> Option<i32> {
    let mut stack = Vec::with_capacity(line.len());

    for c in line.trim().chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return Some(3);
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return Some(57);
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return Some(1197);
                }
            }
            '>' => {
                if stack.pop() != Some('<') {
                    return Some(25137);
                }
            }
            _ => panic!("Unexpected character: '{}'", c),
        }
    }

    Some(0)
}
