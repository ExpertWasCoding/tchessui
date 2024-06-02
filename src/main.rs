use chess::get_board_vector::get_board;
use chess::print_board::{self, print_board};
fn main() {
    let board = get_board();
    print_board(board);
}
