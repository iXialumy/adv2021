use itertools::Itertools;

fn main() {
    let mut input = include_str!("../../resources/input04.txt").split("\n\n");

    let numbers = input
        .next()
        .unwrap()
        .split(",")
        .map(|num| num.parse::<u32>().unwrap())
        .collect_vec();

    let mut boards = input
                .map(|board|  Board::parse_board(board).unwrap())
                .collect_vec();

    let score = find_first_winning_board(&numbers, &mut boards);

    println!("Score: {}", score);
}

fn find_first_winning_board(numbers: &Vec<u32>, boards: &mut Vec<Board>) -> u32 {
    for n in numbers {
        boards.iter_mut()
            .for_each(|board| board.update_board(n));
        
        let winner_boards = boards.iter()
            .filter(|board| board.won())
            .collect_vec();

        if !winner_boards.is_empty() {
            winner_boards[0].print();
            println!("Board sum:    {}", winner_boards[0].score());
            println!("Calles number {}", *n);
            return winner_boards[0].score() * *n;
        }
    }

    0
}

#[derive(Debug)]
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
    
        Some(Board {board, mask})
    }
    
    fn update_board(&mut self, n: &u32) {
        for i in 0..5 {
            for j in 0..5 {
                if self.board[i][j] == *n {self.mask[i][j] = true}
            }
        }
    }

    fn won(&self) -> bool {
        if self.mask.iter().any(|line| line.iter().all(|&n| n == true)) {
            return true;
        }
    
        for i in 0..5 {
            let mut count = 0;
    
            for j in 0..5 {
                if self.mask[j][i] == true {count += 1}
            }
    
            if count == 5 {return true;}
        }
    
        false
    }

    fn score(&self) -> u32 {
        let score: u32 = self.board.iter()
            .enumerate()
            .map(|(i, line)| line.iter()
                .enumerate()
                .filter(|(j, _)| !self.mask[i][*j])
                .map(|(_, &n)| n as u32)
                .sum::<u32>()
            )
            .sum();

        score
    }

    fn print(&self) {
        let bold = ansi_term::Style::new().bold();
        let neutral = ansi_term::Style::new();

        let out = self.board.iter().zip(self.mask.iter())
            .map(|(lb, lm)| lb.iter().zip(lm.iter())
                .map(|(n, &mask)| {
                    let n = format!("{:>2}", *n);
                    if mask {bold.paint(n)} else {neutral.paint(n)}
                })
                .join(" ")
            ).join("\n");
        println!("{}", out);
    }
}

