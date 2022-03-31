use pancurses::{endwin, initscr, Input};
use rust_tetris::tetris::TetrisGameStage;

fn main() {
    let window = initscr();
    window.refresh();
    window.timeout(10);

    let mut frame_count: i32 = 0;
    let mut tetris_game_stage = TetrisGameStage::new();
    window.keypad(true);

    loop {
        if frame_count % 60 == 0 {
            tetris_game_stage.fall_proc();
            frame_count = 0;
        }

        tetris_game_stage.clear_oprated_tetrimino();
        tetris_game_stage.draw();
        window.mvprintw(0, 0, tetris_game_stage.output_field_str());

        match window.getch() {
            Some(Input::Character(c)) => {
                if c == 'q' {
                    break;
                } else if c == 'r' {
                    tetris_game_stage.rotate_proc();
                }
            }
            Some(Input::KeyLeft) => tetris_game_stage.left_proc(),
            Some(Input::KeyRight) => tetris_game_stage.right_proc(),
            Some(Input::KeyDown) => {
                tetris_game_stage.fall_proc();
                if !tetris_game_stage.reset_operated_tetrimino() {
                    println!("Game Over");
                    break;
                }
            }
            Some(Input::KeyUp) => {
                tetris_game_stage.up_proc();
                if !tetris_game_stage.reset_operated_tetrimino() {
                    println!("Game Over");
                    break;
                }
            }
            _ => (),
        }

        frame_count += 1;
    }
    endwin();
}
