use chess::board::get_board;
use chess::pieces::get_pieces;
use chess::print_board::print_board;
fn main() {
    let mut board = get_board();
    let board_pieces = get_pieces();
    board[0] = board_pieces[1].clone();
    board[1] = board_pieces[0].clone();
    board[6] = board_pieces[2].clone();
    board[7] = board_pieces[3].clone();

    print_board(board);
}
//TODO, add notation to board i*n
