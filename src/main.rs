mod tetris;

use pancurses::{endwin, initscr};
use tetris::TetrisGameStage;

fn main() {
    let tetris_game_stage = TetrisGameStage::new();
    println!("{:?}", tetris_game_stage);
    tetris_game_stage.print_field();

    // let window = initscr();
    // window.refresh();
    // window.getch();
    // endwin();
}
