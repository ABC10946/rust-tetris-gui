mod tetris;

use pancurses::{endwin, initscr};
use tetris::{Direction, TetriminoKind, TetrisGameStage};

fn main() {
    let mut tetris_game_stage = TetrisGameStage::new();
    println!("--------------tetrimino display test---------------");
    println!("I Tetrimino");
    tetris_game_stage.put_tetrimino(TetriminoKind::TetI, Direction::Down, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    tetris_game_stage.put_tetrimino(TetriminoKind::TetI, Direction::Left, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    tetris_game_stage.put_tetrimino(TetriminoKind::TetI, Direction::Right, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    tetris_game_stage.put_tetrimino(TetriminoKind::TetI, Direction::Up, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    println!("O Tetrimino");
    tetris_game_stage.put_tetrimino(TetriminoKind::TetO, Direction::Down, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    tetris_game_stage.put_tetrimino(TetriminoKind::TetO, Direction::Left, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    tetris_game_stage.put_tetrimino(TetriminoKind::TetO, Direction::Right, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    tetris_game_stage.put_tetrimino(TetriminoKind::TetO, Direction::Up, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    println!("S Tetrimino");
    tetris_game_stage.put_tetrimino(TetriminoKind::TetS, Direction::Down, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    tetris_game_stage.put_tetrimino(TetriminoKind::TetS, Direction::Left, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    tetris_game_stage.put_tetrimino(TetriminoKind::TetS, Direction::Right, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    tetris_game_stage.put_tetrimino(TetriminoKind::TetS, Direction::Up, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    println!("Z Tetrimino");
    tetris_game_stage.put_tetrimino(TetriminoKind::TetZ, Direction::Down, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    tetris_game_stage.put_tetrimino(TetriminoKind::TetZ, Direction::Left, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    tetris_game_stage.put_tetrimino(TetriminoKind::TetZ, Direction::Right, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    tetris_game_stage.put_tetrimino(TetriminoKind::TetZ, Direction::Up, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    println!("J Tetrimino");
    tetris_game_stage.put_tetrimino(TetriminoKind::TetJ, Direction::Down, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    tetris_game_stage.put_tetrimino(TetriminoKind::TetJ, Direction::Left, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    tetris_game_stage.put_tetrimino(TetriminoKind::TetJ, Direction::Right, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    tetris_game_stage.put_tetrimino(TetriminoKind::TetJ, Direction::Up, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    println!("L Tetrimino");
    tetris_game_stage.put_tetrimino(TetriminoKind::TetL, Direction::Down, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    tetris_game_stage.put_tetrimino(TetriminoKind::TetL, Direction::Left, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    tetris_game_stage.put_tetrimino(TetriminoKind::TetL, Direction::Right, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    tetris_game_stage.put_tetrimino(TetriminoKind::TetL, Direction::Up, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    println!("T Tetrimino");
    tetris_game_stage.put_tetrimino(TetriminoKind::TetT, Direction::Down, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    tetris_game_stage.put_tetrimino(TetriminoKind::TetT, Direction::Left, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    tetris_game_stage.put_tetrimino(TetriminoKind::TetT, Direction::Right, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();

    tetris_game_stage.put_tetrimino(TetriminoKind::TetT, Direction::Up, 5, 5);
    println!("{}", tetris_game_stage.output_field_str());
    tetris_game_stage.clear_oprated_tetrimino();
    // let window = initscr();
    // window.refresh();
    // window.getch();
    // endwin();
}
