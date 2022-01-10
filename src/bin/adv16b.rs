use itertools::Itertools;

fn main() {
    let input = include_str!("../../resources/input16.txt");
    let str = hex_str_to_bin_str(input);

    let (value, _) = parse_packet(&str);
    println!("{}", value);
}

fn parse_packet(str: &str) -> (usize, usize) {
    let _version = usize::from_str_radix(&str[0..3], 2).unwrap();
    let type_id = usize::from_str_radix(&str[3..6], 2).unwrap();

    match type_id {
        0..=3 | 5..=7 => {
            let f = combinator_function(type_id);
            let (value, offset) = parse_operator(&str[6..], f);
            (value, offset + 6)
        }
        4 => {
            // Literal
            let (value, offset) = parse_literal(&str[6..]);
            (value, offset + 6)
        }
        _ => {
            panic!("Unknown type_id: {}", type_id);
        }
    }
}

fn hex_str_to_bin_str(str: &str) -> String {
    str.chars()
        .map(|c| match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!("Unexpected char :'{}'", c),
        })
        .join("")
}

/// Parses a literal
///
/// returns literal and offset
fn parse_literal(str: &str) -> (usize, usize) {
    let mut number = String::new();
    let mut offset = 0;
    loop {
        // Parse groups of 5 bits with their first bit signaling wether they are the last group or not
        let control = &str[offset..=offset];
        number += &str[offset + 1..offset + 5];
        offset += 5;
        if control == "0" {
            break;
        }
    }

    let literal = usize::from_str_radix(&number, 2).unwrap();
    (literal, offset)
}

fn parse_operator<F: Fn(&[usize]) -> usize>(str: &str, f: F) -> (usize, usize) {
    let length_type_id = &str[..1];
    let length_size: usize = match length_type_id {
        "0" => 15,
        "1" => 11,
        _ => panic!("{} is not a valid length_type_id", length_type_id),
    };

    let mut offset = 1;
    let length_bits = &str[offset..length_size + offset]; // plus one because of the offset for the length_type_id
    offset += length_size;
    let length = usize::from_str_radix(length_bits, 2).unwrap();
    let subpackets_offset = offset;

    let mut packets = vec![];
    if length_size == 15 {
        while offset < length + subpackets_offset {
            let (value, parsed_offset) = parse_packet(&str[offset..]);
            packets.push(value);
            offset += parsed_offset;
        }
    } else {
        for _ in 0..length {
            let (value, parsed_offset) = parse_packet(&str[offset..]);
            packets.push(value);
            offset += parsed_offset;
        }
    }

    let value = f(&packets);
    (value, offset)
}

fn combinator_function(id: usize) -> impl Fn(&[usize]) -> usize {
    match id {
        0 => {
            // Sum Operator
            |values: &[usize]| values.iter().sum()
        }
        1 => {
            // Sum Operator
            |values: &[usize]| values.iter().product()
        }
        2 => {
            // Min Operator
            |values: &[usize]| values.iter().min().unwrap().to_owned()
        }
        3 => {
            // Min Operator
            |values: &[usize]| values.iter().max().unwrap().to_owned()
        }
        5 => {
            // Min Operator
            |values: &[usize]| {
                if values[0] > values[1] {
                    1
                } else {
                    0
                }
            }
        }
        6 => {
            // Min Operator
            |values: &[usize]| {
                if values[0] < values[1] {
                    1
                } else {
                    0
                }
            }
        }
        7 => {
            // Min Operator
            |values: &[usize]| {
                if values[0] == values[1] {
                    1
                } else {
                    0
                }
            }
        }
        _ => {
            panic!("Unreachable Type ID: {}", id);
        }
    }
}
