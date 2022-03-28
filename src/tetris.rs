#[derive(Debug)]
pub enum TetriminoKind {
    TetI,
    TetO,
    TetS,
    TetZ,
    TetJ,
    TetL,
    TetT,
}

#[derive(Copy, Clone, Debug)]
pub enum BlockKind {
    Space,
    Block,
    Operating,
    Wall,
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct TetrisGameStage {
    field: [[BlockKind; 12]; 21],
    next_op_tet: TetriminoKind,
    op_tet: OperateTet,
}

#[derive(Debug)]
pub struct OperateTet {
    x: i32,
    y: i32,
    kind: TetriminoKind,
    rotation_id: Direction,
}

impl TetrisGameStage {
    pub fn new() -> TetrisGameStage {
        TetrisGameStage {
            field: initField(),
            next_op_tet: TetriminoKind::TetI,
            op_tet: OperateTet {
                x: 0,
                y: 0,
                kind: TetriminoKind::TetJ,
                rotation_id: Direction::Up,
            },
        }
    }
}

fn initField() -> [[BlockKind; 12]; 21] {
    [[BlockKind::Space; 12]; 21]
}
