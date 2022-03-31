use crate::tetrimino::{tetriminos, Direction, TetriminoKind};

const WIDTH: usize = 12;
const HEIGHT: usize = 21;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlockKind {
    Space,
    Block,
    Operating,
    Wall,
}

#[derive(Debug)]
pub struct TetrisGameStage {
    field: Vec<Vec<BlockKind>>,
    next_op_tet: TetriminoKind,
    op_tet: OperateTet,
}

#[derive(Debug, Clone)]
pub struct OperateTet {
    x: usize,
    y: usize,
    kind: TetriminoKind,
    direction: Direction,
}

impl TetrisGameStage {
    pub fn new() -> TetrisGameStage {
        TetrisGameStage {
            field: init_field(),
            next_op_tet: TetriminoKind::TetI,
            op_tet: OperateTet {
                x: 5,
                y: 0,
                kind: TetriminoKind::TetJ,
                direction: Direction::Up,
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

    fn setable_operated_tet(&self, op_tet: OperateTet) -> bool {
        let tetrimino = tetriminos(op_tet.kind, op_tet.direction);
        let x = op_tet.x;
        let y = op_tet.y;

        if self.field[y + tetrimino[0][1]][x + tetrimino[0][0]] == BlockKind::Block
            || self.field[y + tetrimino[1][1]][x + tetrimino[1][0]] == BlockKind::Block
            || self.field[y + tetrimino[2][1]][x + tetrimino[2][0]] == BlockKind::Block
            || self.field[y + tetrimino[3][1]][x + tetrimino[3][0]] == BlockKind::Block
            || self.field[y + tetrimino[0][1]][x + tetrimino[0][0]] == BlockKind::Wall
            || self.field[y + tetrimino[1][1]][x + tetrimino[1][0]] == BlockKind::Wall
            || self.field[y + tetrimino[2][1]][x + tetrimino[2][0]] == BlockKind::Wall
            || self.field[y + tetrimino[3][1]][x + tetrimino[3][0]] == BlockKind::Wall
        {
            false
        } else {
            true
        }
    }

    fn reset_operated_tetrimino(&mut self) {
        self.op_tet.x = 5;
        self.op_tet.y = 0;
        self.op_tet.kind = self.next_op_tet;
        self.op_tet.direction = Direction::Up;

        if !self.setable_operated_tet(self.op_tet.clone()) {
            println!("Game Over");
        }
    }

    fn change_to_block(&mut self) {
        let tetrimino = tetriminos(self.op_tet.kind.clone(), self.op_tet.direction.clone());
        for th in 0..4 {
            let x_offset = tetrimino[th][0];
            let y_offset = tetrimino[th][1];

            self.field[self.op_tet.y + y_offset][self.op_tet.x + x_offset] = BlockKind::Block;
        }
    }

    pub fn draw(&mut self) {
        self.put_tetrimino(
            self.op_tet.kind.clone(),
            self.op_tet.direction.clone(),
            self.op_tet.x,
            self.op_tet.y,
        );
    }

    pub fn fall_proc(&mut self) {
        self.op_tet.y += 1;
        if !self.setable_operated_tet(self.op_tet.clone()) {
            self.op_tet.y -= 1;
            self.change_to_block();
            self.reset_operated_tetrimino();
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
