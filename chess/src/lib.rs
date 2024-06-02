pub mod get_board_vector {
    pub fn get_board() -> Vec<[&'static str; 8]> {
        let mut board = Vec::new();
        for _ in 0..4 {
            let file_odd = ["█", " ", "█", " ", "█", " ", "█", " "];
            let file_even = [" ", "█", " ", "█", " ", "█", " ", "█"];
            board.push(file_odd);
            board.push(file_even);
        }
        return board;
    }
}

pub mod print_board {
    pub fn print_board(board: Vec<[&str; 8]>) {
        for ranks in board {
            for squares in ranks {
                print!("{}", squares);
            }
            print!("\n");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(get_board_vector::get_board().len(), 8);
    }
}
