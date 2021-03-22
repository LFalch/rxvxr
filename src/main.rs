use rxvxr::Board;

fn main() {
    let mut board = Board::new();
    board.add_log(4, 0);
    board.add_log(1, 2);
    board.add_log(3, 2);
    board.add_log(4, 2);
    board.add_log(0, 4);
    board.add_log(1, 4);
    board.add_log(1, 5);
    board.add_log(3, 5);
    board.add_log(0, 6);
    dbg!(board.set_plank(4, 0, 4, 2));
    dbg!(board.set_plank(3, 2, 4, 2));
    dbg!(board.set_plank(1, 2, 1, 4));
    dbg!(board.set_plank(3, 2, 3, 5));
    dbg!(board.set_plank(0, 4, 1, 4));
    println!("{}", board);
}
