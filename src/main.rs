#![allow(dead_code)]
#![allow(unused_variables)]

use std::{io, fs, time::Instant};

type U8_9 = [u8; 9];
type Pos = (usize, usize); 

#[derive(Debug)]
struct Board {
    board: [u8; 81],
    modify: Vec<Pos>,
}

impl Board {
    fn get(&self, x: usize, y: usize) -> u8 {
        return self.board[y * 9 + x];
    }

    fn get_modify_nth_mut(&mut self, i: usize) -> &mut u8 {
        let pos = self.modify[i];
        return self.board.get_mut(pos.1 * 9 + pos.0).unwrap();
    }

    fn set(&mut self, x: usize, y: usize, val: u8) {
        self.board[(y * 9 + x) as usize] = val;
    }

    fn get_row(&self, row_i: usize) -> U8_9 {
        let mut row: U8_9 = [0; 9];

        for x in 0..9 {
            row[x] = self.get(x, row_i) as u8;
        }

        return row;
    }

    fn get_column(&self, col_i: usize) -> U8_9 {
        let mut column: U8_9 = [0; 9];

        for y in 0..9 {
            column[y] = self.get(col_i, y);
        }

        return column;
    }


    fn get_square(&self, sq_i: usize) -> U8_9 {
        let mut square: U8_9 = [0; 9];

        let x_offset = (sq_i % 3) * 3;
        let y_offset = (sq_i / 3) * 3;

        for (x_i, x) in (x_offset..x_offset + 3).enumerate() {
            for (y_i, y) in (y_offset..y_offset + 3).enumerate() {
                square[y_i * 3 + x_i] = self.get(x, y);
            }
        }

        return square;
    }

    fn get_square_from_pos(&self, x: usize, y: usize) -> U8_9 {
        let square_x = x / 3;
        let square_y = y / 3;

        return self.get_square(square_y * 3 + square_x);
    }

    fn is_pos_correct(&self, x: usize, y: usize) -> bool {
        let is_row_correct = Board::is_array_unique( self.get_row(y) );
        let is_column_correct = Board::is_array_unique( self.get_column(x) );
        let is_square_correct = Board::is_array_unique( self.get_square_from_pos(x, y) );

        return is_row_correct && is_column_correct && is_square_correct;
    }

    fn solve(&mut self) -> Result<(), ()> {
        let backup = self.board;

        let mut modify_i = 0;
        while modify_i <= self.modify.len() - 1 {

            let (x, y) = self.modify[modify_i];
            let val = self.get_modify_nth_mut(modify_i);
            

            *val += 1;
            if *val == 10 {
                if modify_i == 0 {
                    self.board = backup;
                    return Err(());
                }
                *val = 0;
                modify_i -= 1;
                continue;
            }

            if self.is_pos_correct(x, y) {
                modify_i += 1;
            }
        }

        return Ok(());
    }
    
    fn is_array_unique(array: U8_9) -> bool {
        let mut num_count: [u8; 10] = [0; 10];

        for n in array {
            num_count[n as usize] += 1;
        }

        for i in 1..10 {
            if num_count[i] > 1 {
                return false;
            }
        }

        return true;
    }
    
    fn from_file(fname: &str) -> Result<Board, io::Error> {
        let data_str = fs::read_to_string(fname)?;
        let mut board_data: [u8; 81] = [0; 81];
        let mut modify_data: Vec<Pos> = Vec::new();

        let mut count: usize = 0;
        for c in data_str.chars() {
            if c == '\n' || c == '\r' {
                continue;
            }

            let num: u8 = match c.to_digit(10) {
                Some(digit) => digit as u8,
                None => {
                    modify_data.push((count % 9, count / 9));
                    0
                },
            };
            
            board_data[count] = num;
            count += 1;
        }

        return Ok( Board {
            board: board_data,
            modify: modify_data }
        );
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut board_txt = String::new();

        for (i, &num) in self.board.iter().enumerate() {
            if num == 0 {
                board_txt.push_str("_ ");
            } else {
                board_txt.push_str(format!("{} ", num.to_string()).as_str());
            }

            if (i + 1) % 9 == 0 && i != 80 {
                board_txt.push('\n');
            }
        }

        write!(f, "{}", board_txt)
    }
}

fn main() {
    let now = Instant::now();

    let mut sudoku = Board::from_file("board.txt").expect("An error occured when constructing a board.");
    match sudoku.solve() {
        Ok(_) => { println!("Sudoku solved successfully!") },
        Err(_) => { println!("Sudoku is unsolvable.") },

    }

    println!("{sudoku}\nTime: {dt}", dt = now.elapsed().as_millis() as f64 / 1000f64);

}
