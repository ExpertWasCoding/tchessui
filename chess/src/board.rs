pub fn get_board() -> Vec<[&'static str; 8]> {
    let mut board = Vec::new();
    for _ in 0..4 {
        let file_odd = ["■", "□", "■", "□", "■", "□", "■", "□"];
        let file_even = ["□", "■", "□", "■", "□", "■", "□", "■"];
        board.push(file_odd);
        board.push(file_even);
    }
    return board;
}
