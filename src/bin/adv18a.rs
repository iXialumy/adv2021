#![feature(int_roundings)]

use itertools::Itertools;

#[derive(PartialEq, Eq)]
enum Symbol {
    Open,
    Close,
    Comma,
    Num(u8),
}

impl std::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "["),
            Self::Close => write!(f, "]"),
            Self::Comma => write!(f, ","),
            Self::Num(n) => f.write_fmt(format_args!("{}", n)),
        }
    }
}

type Snailfish = Vec<Symbol>;

fn main() {
    let input = include_str!("../../resources/input18.txt");

    let mag = adv18a(input);
    println!("{mag}");
}

fn adv18a(input: &str) -> u32 {
    let sum = snailsum(input);
    mag(sum)
}

fn snailsum(input: &str) -> Snailfish {
    input
        .lines()
        .map(parse_snailfish_number)
        .reduce(add_snailfish)
        .unwrap()
}

fn parse_snailfish_number(input: &str) -> Snailfish {
    input
        .trim()
        .bytes()
        .map(|c| match c {
            b'[' => Symbol::Open,
            b']' => Symbol::Close,
            b',' => Symbol::Comma,
            b'0'..=b'9' => Symbol::Num(c - b'0'),
            _ => panic!("Unexpected character: {}", c),
        })
        .collect()
}

fn reduce_snailfish(snailfish: &mut Snailfish) {
    let mut changed = true;
    while changed {
        changed = try_explode(snailfish);
        if changed {
            continue;
        }
        changed |= try_split(snailfish);
    }
}

fn try_explode(snailfish: &mut Snailfish) -> bool {
    let depths = snailfish.iter().scan(0, |depth, s| {
        match s {
            Symbol::Open => *depth += 1,
            Symbol::Close => *depth -= 1,
            _ => {}
        }
        Some(*depth)
    });
    if let Some((index, (_, depth))) =
        snailfish
            .windows(5)
            .zip(depths)
            .enumerate()
            .find(|(_, (symbol, depth))| {
                *depth > 4
                    && matches!(
                        symbol,
                        [
                            Symbol::Open,
                            Symbol::Num(_),
                            Symbol::Comma,
                            Symbol::Num(_),
                            Symbol::Close
                        ]
                    )
            })
    {
        let left = match snailfish[index + 1] {
            Symbol::Num(n) => n,
            _ => panic!(
                "Not a number: '{:?}' at index {}",
                snailfish[index + 2],
                index + 2
            ),
        };

        let right = match snailfish[index + 3] {
            Symbol::Num(n) => n,
            _ => panic!(),
        };

        // Add left value to first num to the left
        if index > depth {
            for n in snailfish[..index + 1].iter_mut().rev() {
                match n {
                    Symbol::Num(num) => {
                        *num += left;
                        break;
                    }
                    _ => continue,
                }
            }
        }

        for _ in 0..5 {
            snailfish.remove(index);
        }
        snailfish.insert(index, Symbol::Num(0));

        // Add right value to first num to the right
        for n in snailfish[index + 2..].iter_mut() {
            match n {
                Symbol::Num(num) => {
                    *num += right;
                    break;
                }
                _ => continue,
            }
        }

        true
    } else {
        false
    }
}

fn try_split(snailfish: &mut Snailfish) -> bool {
    if let Some((index, Symbol::Num(n))) = snailfish.iter().enumerate().find(|(_, symbol)| {
        if let Symbol::Num(n) = symbol {
            *n > 9
        } else {
            false
        }
    }) {
        let floor = n.div_floor(2);
        let ceil = n.div_ceil(2);

        snailfish.remove(index);

        // Insert elements from last to first
        snailfish.insert(index, Symbol::Close);
        snailfish.insert(index, Symbol::Num(ceil));
        snailfish.insert(index, Symbol::Comma);
        snailfish.insert(index, Symbol::Num(floor));
        snailfish.insert(index, Symbol::Open);

        true
    } else {
        false
    }
}

fn add_snailfish(a: Snailfish, b: Snailfish) -> Snailfish {
    let mut acc = Vec::with_capacity(a.len() + b.len() + 3);
    acc.push(Symbol::Open);
    acc.extend(a);
    acc.push(Symbol::Comma);
    acc.extend(b);
    acc.push(Symbol::Close);

    reduce_snailfish(&mut acc);

    acc
}

fn mag(snailfish: Snailfish) -> u32 {
    let multipliers = snailfish
        .iter()
        .scan(1, |multiplier, symbol| {
            match symbol {
                Symbol::Open => *multiplier *= 3,
                Symbol::Comma => *multiplier = (*multiplier / 3) * 2,
                Symbol::Close => *multiplier /= 2,
                Symbol::Num(_) => {}
            }
            Some(*multiplier)
        })
        .collect_vec();
    snailfish
        .iter()
        .zip(multipliers.iter())
        .map(|(symbol, multiplier)| match symbol {
            Symbol::Num(n) => *n as u32 * multiplier,
            _ => 0,
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_explode_right() {
        let mut actual = parse_snailfish_number("[[[[[9,8],1],2],3],4]");
        let expected = parse_snailfish_number("[[[[0,9],2],3],4]");

        (try_explode(&mut actual));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_explode_left() {
        let mut actual = parse_snailfish_number("[7,[6,[5,[4,[3,2]]]]]");
        let expected = parse_snailfish_number("[7,[6,[5,[7,0]]]]");

        (try_explode(&mut actual));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_split() {
        let mut actual = vec![Symbol::Num(11)];
        let expected = parse_snailfish_number("[5,6]");

        try_split(&mut actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_reduce() {
        let mut actual = parse_snailfish_number("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        let expected = parse_snailfish_number("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

        reduce_snailfish(&mut actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_mag() {
        test_mag_(
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
            4140,
        );
        test_mag_("[[9,1],[1,9]]", 129);
        test_mag_("[[1,2],[[3,4],5]]", 143);
        test_mag_("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445);
        test_mag_("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791);
        test_mag_("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137);
        test_mag_(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            3488,
        );
    }

    fn test_mag_(input: &str, expected: u32) {
        let input = parse_snailfish_number(input);

        let actual = mag(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_add() {
        let a = parse_snailfish_number("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let b = parse_snailfish_number("[1,1]");

        let expected = parse_snailfish_number("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        let actual = add_snailfish(a, b);
        assert_eq!(expected, actual);
    }

    fn print_snailfish(snailfish: &Snailfish) {
        let output = snailfish
            .iter()
            .map(|c| match c {
                Symbol::Open => "[".to_string(),
                Symbol::Close => "]".to_string(),
                Symbol::Comma => ",".to_string(),
                Symbol::Num(n) => n.to_string(),
            })
            .collect::<String>();
        println!("{output}");
    }

    #[test]
    fn test_snailsum() {
        let input = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
        [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]";

        let expected =
            parse_snailfish_number("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");
        let actual = snailsum(input);
        print_snailfish(&expected);
        print_snailfish(&actual);
        assert_eq!(expected, actual);
    }
}
