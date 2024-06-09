use crate::board::{Piece, BLACK_SQUARE, WHITE_SQUARE};
use crate::pieces;
use std::usize;

pub fn move_piece(input: String, mut board: Vec<Vec<Piece>>) -> Vec<Vec<Piece>> {
    let mut updated_pawn_move = board.clone(); // Make this mutable
    for (index, ch) in input.chars().enumerate() {
        if index == 0 && ('a'..='z').contains(&ch) {
            println!("First letter exists and is lowercase");
            if let Some(second_ch) = input.chars().nth(1) {
                if second_ch.is_digit(10) {
                    updated_pawn_move =
                        pawn_move(board.clone(), second_ch.to_digit(10).unwrap() as usize, ch);
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

fn pawn_move(mut board: Vec<Vec<Piece>>, position: usize, file: char) -> Vec<Vec<Piece>> {
    let dest_row = position - 1;
    let dest_col = letter_to_number(file);

    board[dest_row][dest_col] = board[dest_row - 2][dest_col];

    board[dest_row - 2][dest_col] = BLACK_SQUARE;

    return board;
}

fn letter_to_number(letter: char) -> usize {
    match letter {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        _ => 0,
    }
}
