use std::io::BufRead;

extern crate rand;
use rand::Rng;

#[derive(PartialEq, Clone, Copy)]
enum BoardCell {
    Full(bool),
    Empty,
    Invalid
}

const DIMENSION: usize = 3;

struct Board {
    board : [[BoardCell; DIMENSION]; DIMENSION],
}

impl Board {
    
    fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x >= DIMENSION || y >= DIMENSION
    }

    fn get(&self, x: usize, y: usize) -> BoardCell {
        if self.is_in_bounds(x, y) {
            return BoardCell::Invalid;
        }

        self.board[x][y]
    }

    fn who_won(&self) -> Option<bool> {
        // Check rows
        for y in 0..2 {
            let a = self.get(0, y);
            let b = self.get(1, y);
            let c = self.get(2, y);

            if a != BoardCell::Empty && a == b && b == c {
                return Option::Some(match a {BoardCell::Full(x) => x, _ => false});
            }
        }

        // Check columns
        for x in 0..2 {
            let a = self.get(x, 0);
            let b = self.get(x, 1);
            let c = self.get(x, 2);

            if a != BoardCell::Empty && a == b && b == c {
                return Option::Some(match a {BoardCell::Full(x) => x, _ => false});
            }
        }

        // Check diagonals
        {
            let a = self.get(0, 0);
            let b = self.get(1, 1);
            let c = self.get(2, 2);

            if a != BoardCell::Empty && a == b && b == c {
                return Option::Some(match a {BoardCell::Full(x) => x, _ => false});
            }
        }
        {
            let a = self.get(0, 2);
            let b = self.get(1, 1);
            let c = self.get(2, 0);

            if a != BoardCell::Empty && a == b && b == c {
                return Option::Some(match a {BoardCell::Full(x) => x, _ => false});
            }
        }

        Option::None
    }

    fn make_move(&mut self, x: usize, y: usize, val: bool) {
        if self.is_in_bounds(x, y) {
            println!("Invalid move");
            return;
        }

        self.board[x][y] = BoardCell::Full(val);
    }
}

fn draw_board(board: &Board) {
    for x in 0..DIMENSION {
        for y in 0..DIMENSION {
            let cell = board.get(x, y);

            let c = match cell {
                BoardCell::Full(s) => { 
                    match s { true => "X", false => "O"} 
                },
                BoardCell::Empty => " ",
                BoardCell::Invalid => "?",
            };

            print!("|{}|", c);
        }
        println!("\n---------");
    }
}

fn get_input() -> String {
        let stdin = std::io::stdin();
        let stdin = stdin.lock();
        let mut lines = stdin.lines();

        match lines.next().unwrap() {
            Ok(x) => x,
            _ => "".to_string(),
        }
}

fn process_input(input: &String) -> Option<(usize, usize)> {

    if input.len() >= 3 {
        let first_space = match input.find(' ') { Option::Some(x) => x, Option::None => return Option::None };

        let x: usize = match (&input[0..first_space]).parse() {
            Result::Ok(x) => x,
            Err(e) => { println!("{}", e); return Option::None; },
        };

        let y: usize = match (&input[first_space+1..input.len()]).parse()  {
            Result::Ok(x) => x,
            Err(e) => { println!("{}", e); return Option::None; },
        };

        return Option::Some((x, y));
    }

    Option::None
}

fn ai_make_move(board: &mut Board) {

    let mut rng = rand::thread_rng();
    loop {
        let x = rng.gen::<usize>() % DIMENSION;
        let y = rng.gen::<usize>() % DIMENSION;

        if board.get(x, y) == BoardCell::Empty {
            board.make_move(x, y, false);
            return;
        }
    }
}

fn main() {
    let mut board = Board{ board: [[BoardCell::Empty; DIMENSION]; DIMENSION]};

    loop {
        draw_board(&board);

        loop {
            println!("Input: ");
            let ans = get_input();
            let ans = process_input(&ans);

            match ans {
                Some((x, y)) => {
                    println!("You move: {} {}", x, y);
                    board.make_move(x, y, true);
                    break;
                },
                None => {
                    println!("Ain't gonna do shit.");
                }
            };
        }

        match board.who_won() {
            Option::Some(x) => {
                println!("{} has won!", x);
                break;
            },
            None => {}
        }

        ai_make_move(&mut board);

        match board.who_won() {
            Option::Some(x) => {
                println!("{} has won!", x);
                break;
            },
            None => {}
        }
    }
}
