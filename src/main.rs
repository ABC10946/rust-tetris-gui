// use pancurses::{endwin, initscr, Input};
use rust_tetris_gui::tetris::*;
use nannou::prelude::*;
// use nannou::event::Key::*;

const CELL_WIDTH: f32 = 20.0;
const CELL_HEIGHT: f32 = 20.0;
const WINDOW_WIDTH: u32 = 480;
const WINDOW_HEIGHT: u32 = 480;


fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::rate_fps(1.0)) // 現在これは機能しない(RefreshSyncと同じ機能になる)
        .update(update)
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .run();
}

struct Model {
    tetris_game_stage: TetrisGameStage,
    frame_count: u32,
    frame_rate: u32
}

fn model(_app: &App) -> Model {
    _app.new_window().event(event).view(view).build().unwrap();
    let mut tetris_game_stage = TetrisGameStage::new();
    tetris_game_stage.reset_operated_tetrimino();
    tetris_game_stage.draw();
    Model { tetris_game_stage, frame_count: 0, frame_rate: 2}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.frame_count += 1;
    if _model.frame_count > 60/_model.frame_rate { // rate_fpsが現在機能しないため暫定処理(nannou #456)
        _model.tetris_game_stage.fall_proc();
        _model.frame_count = 0;
    }
    _model.tetris_game_stage.clear_operated_tetrimino();
    _model.tetris_game_stage.draw();
}


fn view(_app: &App, _model: &Model, frame: Frame){
    let draw = _app.draw();
    draw.background().color(BLACK);
    for y in 0..21 {
        for x in 0..12 {
            match _model.tetris_game_stage.get_field()[y][x] {
                BlockKind::Space => {
                    draw.rect().w(CELL_WIDTH).h(CELL_HEIGHT).x_y(-120.0 + x as f32 * CELL_WIDTH, 210.0 -1.0 * y as f32 * CELL_HEIGHT).color(GRAY);
                },
                BlockKind::Block => {
                    draw.rect().stroke_weight(1.0).w(CELL_WIDTH).h(CELL_HEIGHT).x_y(-120.0 + x as f32 * CELL_WIDTH, 210.0 -1.0 * y as f32 * CELL_HEIGHT).color(BLUE);
                },
                BlockKind::Operating => {
                    draw.rect().stroke_weight(1.0).w(CELL_WIDTH).h(CELL_HEIGHT).x_y(-120.0 + x as f32 * CELL_WIDTH, 210.0 -1.0 * y as f32 * CELL_HEIGHT).color(RED);
                },
                BlockKind::Wall => {
                    draw.rect().stroke_weight(1.0).w(CELL_WIDTH).h(CELL_HEIGHT).x_y(-120.0 + x as f32 * CELL_WIDTH, 210.0 -1.0 * y as f32 * CELL_HEIGHT).color(GREEN);
                },
                _ => {}
            }
        }
    }

    draw.to_frame(_app, &frame).unwrap();
}

fn event(_app: &App, _model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => {
            match key {
                Key::R => {
                    _model.tetris_game_stage.rotate_proc();
                }
                Key::Right => {
                    _model.tetris_game_stage.right_proc();
                    println!("{:#?}", key);
                },
                Key::Left => {
                    _model.tetris_game_stage.left_proc();
                    println!("{:#?}", key);
                },
                Key::Up => {
                    _model.tetris_game_stage.up_proc();
                    println!("{:#?}", key);
                    if !_model.tetris_game_stage.reset_operated_tetrimino() {
                        println!("Game Over");
                    }
                },
                Key::Down => {
                    _model.tetris_game_stage.fall_proc();
                    println!("{:#?}", key);
                    // if !_model.tetris_game_stage.reset_operated_tetrimino() {
                    //     println!("Game Over");
                    // }
                },
                _ => {println!("{:#?}", key)}
            }
        },
        _ => {}
    }
}

