use itertools::Itertools;

fn main() {
    // let input = "D2FE28"; // Output: 2021
    let input = include_str!("../../resources/input16.txt");
    let str = hex_str_to_bin_str(input);

    let (version_sum, _) = parse_packet(&str);
    println!("Version sum: {}", version_sum);
}

fn parse_packet(str: &str) -> (usize, usize) {
    let version = usize::from_str_radix(&str[0..3], 2).unwrap();
    let type_id = usize::from_str_radix(&str[3..6], 2).unwrap();

    match type_id {
        4 => {
            // Literal
            let offset = parse_literal(&str[6..]);
            (version, offset + 6)
        }
        _ => {
            // Operator
            let (version_sum, offset) = parse_operator(&str[6..]);
            (version + version_sum, offset + 6)
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

fn parse_literal(str: &str) -> usize {
    let mut offset = 0;
    loop {
        let control = &str[offset..=offset];
        offset += 5;
        if control == "0" {
            break;
        }
    }

    offset
}

fn parse_operator(str: &str) -> (usize, usize) {
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

    let mut acc = 0;
    if length_size == 15 {
        while offset < length + subpackets_offset {
            let (version_sum, parsed_offset) = parse_packet(&str[offset..]);
            acc += version_sum;
            offset += parsed_offset;
        }
    } else {
        for _ in 0..length {
            let (version_sum, parsed_offset) = parse_packet(&str[offset..]);
            acc += version_sum;
            offset += parsed_offset;
        }
    }
    (acc, offset)
}
