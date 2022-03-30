use pancurses::{endwin, initscr};
use rust_tetris::tetrimino::{Direction, TetriminoKind};
use rust_tetris::tetris::{self, OperateTet, TetrisGameStage};

fn main() {
    let mut tetris_game_stage = TetrisGameStage::new();
    for i in 0..40 {
        tetris_game_stage.fall_proc();
        tetris_game_stage.draw();
        println!("{}", tetris_game_stage.output_field_str());
        tetris_game_stage.clear_oprated_tetrimino();
    }

    // let window = initscr();
    // window.refresh();
    // window.getch();
    // endwin();
}
