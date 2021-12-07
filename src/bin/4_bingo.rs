#![feature(fn_traits)]

use advent_of_code::read_input;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
struct Field {
    turn: usize,
    num: u8,
}

impl Field {
    fn new(turn: usize, num: u8) -> Field {
        Field { turn, num }
    }
}

type Fields = [Field; 5];

trait Complete {
    fn complete(&self, turn: usize) -> bool;
}

impl Complete for Fields {
    fn complete(&self, turn: usize) -> bool {
        self.iter().all(|field| field.turn <= turn)
    }
}

#[derive(Copy, Clone, Debug)]
struct Board {
    rows: [Fields; 5],
    cols: [Fields; 5],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            rows: [[Field {
                num: 0,
                turn: usize::MAX,
            }; 5]; 5],
            cols: [[Field {
                num: 0,
                turn: usize::MAX,
            }; 5]; 5],
        }
    }
}

impl Board {
    fn new(string: &str, draw_order: HashMap<u8, usize>) -> Self {
        let mut board = Board::default();
        for (row_index, row) in string.split('\n').enumerate() {
            for (col_index, numstr) in row
                .split(' ')
                .filter(|operand| !operand.is_empty())
                .enumerate()
            {
                let number = numstr.parse::<u8>().unwrap();
                let turn = draw_order.get(&number).unwrap();
                let field = Field::new(*turn, number);

                board.rows[row_index][col_index] = field;
                board.cols[col_index][row_index] = field;
            }
        }

        board
    }

    fn winner(&self, turn: usize) -> bool {
        self.rows.iter().any(|row| row.complete(turn))
            || self.cols.iter().any(|col| col.complete(turn))
    }

    fn sum_of_rest(&self, turn: usize) -> u32 {
        self.rows
            .iter()
            .flat_map(|row| row.iter())
            .filter(|field| field.turn > turn)
            .map(|field| field.num as u32)
            .sum()
    }
}

fn prepare(input: &str) -> (Vec<u8>, Vec<Board>) {
    let split: Vec<&str> = input.splitn(2, '\n').collect();
    let draw_order: Vec<u8> = split[0]
        .split(',')
        .map(|string| string.parse::<u8>().unwrap())
        .collect();
    let draw_order_map: HashMap<u8, usize> = draw_order
        .clone()
        .into_iter()
        .enumerate()
        .map(|(index, num)| (num, index))
        .collect();
    let boards: Vec<Board> = split[1]
        .split("\n\n")
        .map(|board| Board::new(board.trim(), draw_order_map.clone()))
        .collect();

    (draw_order, boards)
}

fn bingo_winner(order: Vec<u8>, boards: Vec<Board>) -> usize {
    for (turn, number) in order.into_iter().enumerate() {
        if let Some(board) = boards.iter().find(|board| board.winner(turn)) {
            return (number as u32 * board.sum_of_rest(turn)) as usize;
        }
    }

    panic!("Should never be reached (1)")
}

fn bingo_loser(order: Vec<u8>, boards: Vec<Board>) -> usize {
    let mut losers = boards;
    for (turn, number) in order.into_iter().enumerate() {
        if losers.len() > 1 {
            losers.retain(|board| !board.winner(turn));
        } else if losers[0].winner(turn) {
            return (number as u32 * losers[0].sum_of_rest(turn)) as usize;
        }
    }

    panic!("Should never be reached (2)")
}

fn main() {
    let (order, boards) = prepare(&*read_input!());
    println!("Winner: {}", bingo_winner(order.clone(), boards.clone()));
    println!("Loser:  {}", bingo_loser(order, boards));
}

#[cfg(test)]
mod tests {
    use crate::{bingo_loser, bingo_winner, prepare};

    const RAW: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_bingo_winner() {
        assert_eq!(4512, std::ops::Fn::call(&bingo_winner, prepare(RAW)));
    }

    #[test]
    fn test_bingo_loser() {
        assert_eq!(1924, std::ops::Fn::call(&bingo_loser, prepare(RAW)));
    }
}
