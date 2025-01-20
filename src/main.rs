use std::fmt;
use std::io;

#[derive(PartialEq, Debug, Clone, Copy)]
enum SquareState {
    Empty,
    Player0,
    Player1,
}

struct GameState {
    board: [[SquareState; 3]; 3],
}

impl fmt::Display for SquareState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SquareState::Empty => write!(f, " "),
            SquareState::Player0 => write!(f, "X"),
            SquareState::Player1 => write!(f, "O"),
        }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (row_idx, row) in self.board.iter().enumerate() {
            for (col_idx, square) in row.iter().enumerate() {
                if col_idx > 0 {
                    write!(f, "|")?;
                }
                write!(f, "{}", square)?;
            }
            writeln!(f)?;

            if row_idx < 2 {
                writeln!(f, "-+-+-")?;
            }
        }
        Ok(())
    }
}

enum GameResult {
    Win0,
    Win1,
    Draw,
}

impl GameState {
    fn check_result(&self) -> Option<GameResult> {
        for row in &self.board {
            if row.iter().all(|square| *square == SquareState::Player0) {
                return Some(GameResult::Win0);
            }
            if row.iter().all(|square| *square == SquareState::Player1) {
                return Some(GameResult::Win1);
            }
        }
        for col_idx in 0..3 {
            if self
                .board
                .iter()
                .all(|row| row[col_idx] == SquareState::Player0)
            {
                return Some(GameResult::Win0);
            }
            if self
                .board
                .iter()
                .all(|row| row[col_idx] == SquareState::Player1)
            {
                return Some(GameResult::Win1);
            }
        }
        if self.board[0][0] == SquareState::Player0
            && self.board[1][1] == SquareState::Player0
            && self.board[2][2] == SquareState::Player0
        {
            return Some(GameResult::Win0);
        }
        if self.board[0][0] == SquareState::Player1
            && self.board[1][1] == SquareState::Player1
            && self.board[2][2] == SquareState::Player1
        {
            return Some(GameResult::Win1);
        }
        if self.board[0][2] == SquareState::Player0
            && self.board[1][1] == SquareState::Player0
            && self.board[2][0] == SquareState::Player0
        {
            return Some(GameResult::Win0);
        }
        if self.board[0][2] == SquareState::Player1
            && self.board[1][1] == SquareState::Player1
            && self.board[2][0] == SquareState::Player1
        {
            return Some(GameResult::Win1);
        }
        if self
            .board
            .iter()
            .all(|row| row.iter().all(|square| *square != SquareState::Empty))
        {
            return Some(GameResult::Draw);
        }
        None
    }

    fn new() -> Self {
        Self {
            board: [[SquareState::Empty; 3]; 3],
        }
    }
}

fn main() {
    let mut game = GameState::new();
    while game.check_result().is_none() {
        println!("Current game state:\n{}", game);
        for player in [SquareState::Player0, SquareState::Player1] {
            println!(
                "Player {} turn. Enter row and column (0-2) separated by space:",
                player
            );
            loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                let coords: Vec<&str> = input.trim().split_whitespace().collect();

                if coords.len() != 2 {
                    println!("Please enter two numbers separated by space");
                    continue;
                }

                let row = match coords[0].parse::<usize>() {
                    Ok(num) if num < 3 => num,
                    _ => {
                        println!("Invalid row number. Must be 0, 1, or 2");
                        continue;
                    }
                };

                let col = match coords[1].parse::<usize>() {
                    Ok(num) if num < 3 => num,
                    _ => {
                        println!("Invalid column number. Must be 0, 1, or 2");
                        continue;
                    }
                };

                if game.board[row][col] == SquareState::Empty {
                    game.board[row][col] = player;
                    break;
                } else {
                    println!("That position is already taken. Try again.");
                }
            }
            println!("Current game state:\n{}", game);
            if let Some(result) = game.check_result() {
                match result {
                    GameResult::Win0 => println!("Player 0 wins!"),
                    GameResult::Win1 => println!("Player 1 wins!"),
                    GameResult::Draw => println!("Draw!"),
                }
                break;
            }
        }
    }
}
