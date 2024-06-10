use crate::pieces::Piece;
pub fn print_board(board: Vec<Vec<Piece>>) {
    for ranks in board.iter().rev() {
        for squares in ranks.iter().rev() {
            print!("{}", squares.symbol);
            print!(" ");
        }
        print!("\n");
    }
}
