use chess::board::get_board;
use chess::pieces::get_pieces;
use chess::print_board::print_board;
fn main() {
    let mut board = get_board();
    let board_pieces = get_pieces();
    print_board(board);
}
//TODO, add notation to board i*n
