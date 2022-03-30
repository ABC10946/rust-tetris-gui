mod tetris;

use pancurses::{endwin, initscr};
use tetris::{Direction, TetriminoKind, TetrisGameStage};

fn main() {
    let mut tetris_game_stage = TetrisGameStage::new();
    tetris_game_stage.put_tetrimino(TetriminoKind::TetT, Direction::Up, 5, 5);
    tetris_game_stage.print_field();

    // let window = initscr();
    // window.refresh();
    // window.getch();
    // endwin();
}
