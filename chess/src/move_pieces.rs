use crate::board::{Piece, BLACK_SQUARE, WHITE_SQUARE};
use crate::pieces::{self, Color};
use std::usize;

pub fn move_piece(
    input: String,
    mut board: Vec<Vec<Piece>>,
    current_turn: Color,
) -> Vec<Vec<Piece>> {
    let mut updated_pawn_move = board.clone(); // Make this mutable
    for (index, ch) in input.chars().enumerate() {
        if index == 0 && ('a'..='z').contains(&ch) {
            println!("First letter exists and is lowercase");
            if let Some(second_ch) = input.chars().nth(1) {
                if second_ch.is_digit(10) {
                    updated_pawn_move = pawn_move(
                        board.clone(),
                        second_ch.to_digit(10).unwrap() as usize,
                        ch,
                        current_turn,
                    );
                } else if second_ch == 'x' {
                    todo!();
                }
            } else {
                println!("No second character");
            }
            break;
        }
    }
    return updated_pawn_move;
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

fn letter_to_number(letter: char) -> usize {
    match letter {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => 404,
    }
}
