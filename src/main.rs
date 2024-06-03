use chess::board::get_board;
use chess::pieces::get_pieces;
use chess::print_board::print_board;
fn main() {
    let board = get_board();
    print_board(board);
    let board_pieces = get_pieces();
}
//TODO, add notation to board i*n
