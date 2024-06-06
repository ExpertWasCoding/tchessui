pub mod board;
pub mod pieces;
pub mod print_board {
    use crate::pieces::Piece;

    pub fn print_board(board: Vec<Vec<Piece>>) {
        for ranks in board {
            for squares in ranks {
                print!("{:?}", squares.name);
                print!(" ");
            }
            print!("\n");
        }
    }
}
