use chess::board::get_board;
use chess::board::Color;
use chess::move_pieces::move_piece;
use chess::pieces::get_pieces;
use chess::print_board::print_board;
use std::io;
use std::thread;
use std::time::Duration;
fn main() {
    let mut current_move = String::new();
    let mut board = get_board();
    let board_pieces = get_pieces();
    board[0] = board_pieces[1].clone();
    board[1] = board_pieces[0].clone();
    board[6] = board_pieces[2].clone();
    board[7] = board_pieces[3].clone();
    print_board(board.clone());
    let mov = String::from("e4");
    let mut current_board = board.clone();
    let mut current_turn: Color = Color::White;
    loop {
        current_board = move_piece(mov.clone(), current_board, current_turn);
        thread::sleep(Duration::from_secs(1));
        print_board(current_board.clone());
        match current_turn {
            Color::White => current_turn = Color::Black,
            Color::Black => current_turn = Color::White,
        }
    }
}
//TODO, add notation to board i*n
//add game loop
