mod tetris;

use pancurses::{endwin, initscr};
use tetris::TetrisGameStage;

fn main() {
    let mut tetrisGameStage = TetrisGameStage::new();
    println!("{:?}", tetrisGameStage);

    // let window = initscr();
    // window.refresh();
    // window.getch();
    // endwin();
}
