pub mod board;
pub mod pieces;
pub mod print_board {
    pub fn print_board(board: Vec<[&str; 8]>) {
        for ranks in board {
            for squares in ranks {
                print!("{}", squares);
                print!(" ");
            }
            print!("\n");
        }
    }
}
