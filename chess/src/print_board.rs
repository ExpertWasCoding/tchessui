use crate::pieces::Piece;
pub fn print_board(board: Vec<Vec<Piece>>) {
    for ranks in board {
        for squares in ranks {
            print!("{}", squares.symbol);
            print!(" ");
        }
        print!("\n");
    }
}
