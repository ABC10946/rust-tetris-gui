use pancurses::{endwin, initscr};
use rust_tetris::tetrimino::{Direction, TetriminoKind};
use rust_tetris::tetris::{self, OperateTet, TetrisGameStage};

fn main() {
    let window = initscr();
    window.refresh();

    let mut tetris_game_stage = TetrisGameStage::new();
    loop {
        window.getch();
        tetris_game_stage.clear_oprated_tetrimino();
        tetris_game_stage.draw();
        window.mvprintw(0, 0, tetris_game_stage.output_field_str());
        tetris_game_stage.fall_proc();
    }
    endwin();
}
