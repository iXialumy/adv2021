#![feature(drain_filter)]

use itertools::Itertools;

fn main() {
    let mut input = include_str!("../../resources/input04.txt").split("\n\n");

    let numbers = input
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse::<u32>().unwrap())
        .collect_vec();

    let boards = input
        .map(|board| Board::parse_board(board).unwrap())
        .collect_vec();

    let score = find_last_winning_board(&numbers, boards);

    println!("Score: {}", score);
}

fn find_last_winning_board(numbers: &[u32], boards: Vec<Board>) -> u32 {
    let mut winning_board = None;
    let mut last_number = 0;

    let mut boards = boards;

    for n in numbers {
        let winners = boards
            .drain_filter(|board| {
                board.update_board(n);
                board.won()
            })
            .collect_vec();

        if winners.last().is_some() {
            winning_board = winners.last().copied();
        }

        last_number = *n;

        if boards.is_empty() {
            break;
        }
    }

    if let Some(board) = winning_board {
        board.print();
        println!("Last number: {}", last_number);
        board.score() * last_number
    } else {
        0
    }
}

#[derive(Debug, Clone, Copy)]
struct Board {
    board: [[u32; 5]; 5],
    mask: [[bool; 5]; 5],
}

impl Board {
    fn parse_board(s: &str) -> Option<Board> {
        let mut board = [[0u32; 5]; 5];
        let mask = [[false; 5]; 5];

        for (i, line) in s.lines().enumerate() {
            for (j, num) in line.split_whitespace().enumerate() {
                board[i][j] = num.parse().unwrap();
            }
        }

        Some(Board { board, mask })
    }

    fn update_board(&mut self, n: &u32) {
        for i in 0..5 {
            for j in 0..5 {
                if self.board[i][j] == *n {
                    self.mask[i][j] = true
                }
            }
        }
    }

    fn won(&self) -> bool {
        if self.mask.iter().any(|line| line.iter().all(|&n| n)) {
            return true;
        }

        for i in 0..5 {
            let mut count = 0;

            for j in 0..5 {
                if self.mask[j][i] {
                    count += 1
                }
            }

            if count == 5 {
                return true;
            }
        }

        false
    }

    fn score(&self) -> u32 {
        let score: u32 = self
            .board
            .iter()
            .enumerate()
            .map(|(i, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(j, _)| !self.mask[i][*j])
                    .map(|(_, &n)| n as u32)
                    .sum::<u32>()
            })
            .sum();

        score
    }

    fn print(&self) {
        let bold = ansi_term::Style::new().bold();
        let neutral = ansi_term::Style::new();

        let out = self
            .board
            .iter()
            .zip(self.mask.iter())
            .map(|(lb, lm)| {
                lb.iter()
                    .zip(lm.iter())
                    .map(|(n, &mask)| {
                        let n = format!("{:>2}", *n);
                        if mask {
                            bold.paint(n)
                        } else {
                            neutral.paint(n)
                        }
                    })
                    .join(" ")
            })
            .join("\n");
        println!("{}", out);
    }
}
