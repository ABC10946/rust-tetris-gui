use std::convert;

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

#[derive(Copy, Clone, Debug, PartialEq)]
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
    field: Vec<Vec<BlockKind>>,
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
            field: init_field(),
            next_op_tet: TetriminoKind::TetI,
            op_tet: OperateTet {
                x: 0,
                y: 0,
                kind: TetriminoKind::TetJ,
                rotation_id: Direction::Up,
            },
        }
    }

    pub fn print_field(&self) {
        for h in 0..21 {
            let convertedLine = self.field[h]
                .iter()
                .map(|ele| convert_blockkind_to_char(&ele))
                .collect::<Vec<String>>()
                .join("");

            println!("{}", convertedLine);
        }
    }
}

fn init_field() -> Vec<Vec<BlockKind>> {
    let mut field = vec![vec![BlockKind::Space; 12]; 21];

    for h in 0..21 {
        for w in 0..12 {
            if w == 0 || w == 12 - 1 || h == 21 - 1 {
                field[h][w] = BlockKind::Wall;
            }
        }
    }
    field
}

// printデバッグ用関数
fn convert_blockkind_to_char(kind: &BlockKind) -> String {
    if *kind == BlockKind::Block {
        "#".to_string()
    } else if *kind == BlockKind::Operating {
        "!".to_string()
    } else if *kind == BlockKind::Wall {
        "+".to_string()
    } else {
        "_".to_string()
    }
}
