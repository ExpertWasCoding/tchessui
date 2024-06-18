use crate::board::{Piece, BLACK_SQUARE, WHITE_SQUARE};
use crate::pieces::{self, Color};
use crate::process_move::{self, process};
use std::usize;
pub struct Current_move_string {
    pub destination_file: Option<char>,
    pub destination_number: Option<char>,
    pub takes_or_not: bool,
    pub castling: bool,
}

impl Current_move_string {
    fn new(input: String) -> Current_move_string {
        let takes_or_not: bool = false;
        let castling: bool = false;
        let destination_number: Option<char>;
        let destination_file: Option<char>;
        match input.len() {
            2 => {
                destination_file = input.chars().next();
                destination_number = input.chars().next();
            }
            3 => {
                todo!()
            }
            4 => {
                todo!()
            }
            5 => {
                todo!()
            }
            _ => {
                todo!()
            }
        }
        Current_move_string {
            destination_file,
            destination_number,
            takes_or_not,
            castling,
        }
    }
}

pub fn move_piece(
    input: String,
    mut board: Vec<Vec<Piece>>,
    current_turn: Color,
) -> Vec<Vec<Piece>> {
    let Current_move = Current_move_string::new(input);
    let processed_board = process(Current_move, board);
    return processed_board;
}

fn pawn_move(
    mut board: Vec<Vec<Piece>>,
    position: usize,
    file: char,
    current_turn: Color,
) -> Vec<Vec<Piece>> {
    let dest_row = position - 1;
    let dest_col = letter_to_number(file);
    if board[dest_row - 2][dest_col].notation == Some('P') {
        board[dest_row][dest_col] = board[dest_row - 2][dest_col];

        board[dest_row - 2][dest_col] = BLACK_SQUARE;
    }
    return board;
}
