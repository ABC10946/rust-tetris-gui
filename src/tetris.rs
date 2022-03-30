const WIDTH: usize = 12;
const HEIGHT: usize = 21;

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

    pub fn output_field_str(&self) -> String {
        let mut field_str = String::new();
        for h in 0..HEIGHT {
            let converted_line = self.field[h]
                .iter()
                .map(|ele| convert_blockkind_to_char(&ele))
                .collect::<Vec<String>>()
                .join("");

            let line = format!("{}\n", converted_line);
            field_str.push_str(&line);
        }
        field_str
    }

    pub fn put_tetrimino(&mut self, kind: TetriminoKind, direction: Direction, x: usize, y: usize) {
        let tetrimino = tetriminos(kind, direction);
        for th in 0..4 {
            let x_offset = tetrimino[th][0];
            let y_offset = tetrimino[th][1];
            self.field[y + y_offset][x + x_offset] = BlockKind::Operating;
        }
    }

    pub fn clear_oprated_tetrimino(&mut self) {
        for h in 0..HEIGHT {
            for w in 0..WIDTH {
                if self.field[h][w] == BlockKind::Operating {
                    self.field[h][w] = BlockKind::Space;
                }
            }
        }
    }
}

fn init_field() -> Vec<Vec<BlockKind>> {
    let mut field = vec![vec![BlockKind::Space; WIDTH]; HEIGHT];

    for h in 0..HEIGHT {
        for w in 0..WIDTH {
            if w == 0 || w == WIDTH - 1 || h == HEIGHT - 1 {
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

fn tetriminos(kind: TetriminoKind, direction: Direction) -> [[usize; 2]; 4] {
    match kind {
        TetriminoKind::TetI => match direction {
            Direction::Down => [[0, 0], [0, 1], [0, 2], [0, 3]],
            Direction::Left => [[0, 0], [1, 0], [2, 0], [3, 0]],
            Direction::Right => [[0, 0], [1, 0], [2, 0], [3, 0]],
            Direction::Up => [[0, 0], [0, 1], [0, 2], [0, 3]],
        },
        TetriminoKind::TetJ => match direction {
            Direction::Down => [[0, 0], [1, 0], [0, 1], [0, 2]],
            Direction::Left => [[0, 0], [1, 0], [2, 0], [2, 1]],
            Direction::Right => [[0, 0], [0, 1], [1, 1], [2, 1]],
            Direction::Up => [[1, 0], [1, 1], [1, 2], [0, 2]],
        },
        TetriminoKind::TetL => match direction {
            Direction::Down => [[0, 0], [1, 0], [1, 1], [1, 2]],
            Direction::Left => [[2, 0], [0, 1], [1, 1], [2, 1]],
            Direction::Right => [[0, 0], [1, 0], [2, 0], [0, 1]],
            Direction::Up => [[0, 0], [0, 1], [0, 2], [1, 2]],
        },
        TetriminoKind::TetO => match direction {
            Direction::Down => [[0, 0], [1, 0], [0, 1], [1, 1]],
            Direction::Left => [[0, 0], [1, 0], [0, 1], [1, 1]],
            Direction::Right => [[0, 0], [1, 0], [0, 1], [1, 1]],
            Direction::Up => [[0, 0], [1, 0], [0, 1], [1, 1]],
        },
        TetriminoKind::TetS => match direction {
            Direction::Down => [[1, 0], [2, 0], [0, 1], [1, 1]],
            Direction::Left => [[0, 0], [0, 1], [1, 1], [1, 2]],
            Direction::Right => [[0, 0], [0, 1], [1, 1], [1, 2]],
            Direction::Up => [[1, 0], [2, 0], [0, 1], [1, 1]],
        },
        TetriminoKind::TetT => match direction {
            Direction::Down => [[0, 0], [1, 0], [2, 0], [1, 1]],
            Direction::Left => [[1, 0], [0, 1], [1, 1], [1, 2]],
            Direction::Right => [[2, 1], [1, 0], [1, 1], [1, 2]],
            Direction::Up => [[1, 1], [0, 2], [1, 2], [2, 2]],
        },
        TetriminoKind::TetZ => match direction {
            Direction::Down => [[0, 0], [1, 0], [1, 1], [2, 1]],
            Direction::Left => [[1, 0], [0, 1], [1, 1], [0, 2]],
            Direction::Right => [[1, 0], [0, 1], [1, 1], [0, 2]],
            Direction::Up => [[0, 0], [1, 0], [1, 1], [2, 1]],
        },
    }
}
