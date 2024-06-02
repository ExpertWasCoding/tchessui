mod get_board_vector {
    pub fn get_board() -> Vec<&'static str> {
        let mut board = Vec::new();
        for _ in 0..=31 {
            board.push("0");
            board.push("1");
        }
        return board;
    }
}

mod print_board {
    pub fn print_board() {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(get_board_vector::get_board().len(), 64);
    }
}
