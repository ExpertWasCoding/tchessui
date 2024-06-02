use chess::get_board_vector::get_board;
fn main() {
    let board = get_board();
    for ranks in board {
        for squares in ranks {
            print!("{}", squares);
        }
        print!("\n");
    }
}
