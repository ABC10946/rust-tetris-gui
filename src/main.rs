mod tetris;

use pancurses::{endwin, initscr};
use tetris::TetrisGameStage;

fn main() {
    let mut tetrisGameStage = TetrisGameStage::new();
    println!("{:?}", tetrisGameStage);
    tetrisGameStage.print_field();

    // let window = initscr();
    // window.refresh();
    // window.getch();
    // endwin();
}
