use crate::tetrimino::{tetriminos, Direction, TetriminoKind, KINDS};
use rand::prelude::random;

const WIDTH: usize = 12;
const HEIGHT: usize = 21;


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlockKind {
    Space,
    Block,
    Operating,
    Wall,
}

type Field = Vec<Vec<BlockKind>>;

#[derive(Debug)]
pub struct TetrisGameStage {
    field: Field,
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
            next_op_tet: random_tetrimino(),
            op_tet: OperateTet {
                x: 5,
                y: 0,
                kind: random_tetrimino(),
                direction: Direction::Up,
            },
        }
    }

    pub fn get_field(&self) -> &Field {
        &self.field
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

    pub fn clear_operated_tetrimino(&mut self) {
        for h in 0..HEIGHT {
            for w in 0..WIDTH {
                if self.field[h][w] == BlockKind::Operating {
                    self.field[h][w] = BlockKind::Space;
                }
            }
        }
    }

    fn setable_operated_tet(&self) -> bool {
        let tetrimino = tetriminos(self.op_tet.kind, self.op_tet.direction);
        let x = self.op_tet.x;
        let y = self.op_tet.y;

        for i in 0..4 {
            if x + tetrimino[i][0] >= WIDTH {
                return false;
            }

            if y + tetrimino[i][1] >= HEIGHT {
                return false;
            }
        }

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

    pub fn reset_operated_tetrimino(&mut self) -> bool {
        self.op_tet.x = 5;
        self.op_tet.y = 0;
        self.op_tet.kind = self.next_op_tet;
        self.op_tet.direction = Direction::Up;

        self.next_op_tet = random_tetrimino();

        self.setable_operated_tet()
    }

    fn change_to_block(&mut self) {
        println!("change to block");
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
        println!("fall_proc");
        if !self.setable_operated_tet() {
            println!("touch ground");
            self.op_tet.y -= 1;
            self.change_to_block();
            self.remove_line_proc();
            self.reset_operated_tetrimino();
        }
    }

    pub fn left_proc(&mut self) {
        self.op_tet.x -= 1;
        if !self.setable_operated_tet() {
            self.op_tet.x += 1;
        }
    }

    pub fn right_proc(&mut self) {
        self.op_tet.x += 1;
        if !self.setable_operated_tet() {
            self.op_tet.x -= 1;
        }
    }

    pub fn up_proc(&mut self) {
        while self.setable_operated_tet() {
            self.op_tet.y += 1;
        }
        self.op_tet.y -= 1;
        self.change_to_block();
        self.remove_line_proc();
    }

    pub fn rotate_proc(&mut self) {
        if self.op_tet.direction == Direction::Up {
            self.op_tet.direction = Direction::Right;
            if !self.setable_operated_tet() {
                self.op_tet.direction = Direction::Up;
            }
        } else if self.op_tet.direction == Direction::Right {
            self.op_tet.direction = Direction::Down;
            if !self.setable_operated_tet() {
                self.op_tet.direction = Direction::Right;
            }
        } else if self.op_tet.direction == Direction::Down {
            self.op_tet.direction = Direction::Left;
            if !self.setable_operated_tet() {
                self.op_tet.direction = Direction::Down;
            }
        } else if self.op_tet.direction == Direction::Left {
            self.op_tet.direction = Direction::Up;
            if !self.setable_operated_tet() {
                self.op_tet.direction = Direction::Left;
            }
        }
    }

    fn is_full_line(&self, line_num: usize) -> bool {
        let mut count: u8 = 0;
        for w in 0..WIDTH {
            if self.field[line_num][w] == BlockKind::Block {
                count += 1;
            }
        }
        count == 10
    }

    fn delete_line(&mut self, line_num: usize) {
        for w in 0..WIDTH {
            if self.field[line_num][w] == BlockKind::Block {
                self.field[line_num][w] = BlockKind::Space;
            }
        }
    }

    fn move_all_block(&mut self, pivot: usize, step: usize) {
        for h in (0..HEIGHT).rev() {
            for w in 0..WIDTH {
                if self.field[h][w] == BlockKind::Block && h < pivot {
                    self.field[h][w] = BlockKind::Space;
                    self.field[h + step][w] = BlockKind::Block;
                }
            }
        }
    }

    pub fn remove_line_proc(&mut self) {
        for h in 0..HEIGHT {
            if self.is_full_line(h) {
                self.delete_line(h);
                self.move_all_block(h, 1);
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

fn random_tetrimino() -> TetriminoKind {
    let tetrimino_kind_id = (random::<u8>() % 7) as usize;
    KINDS[tetrimino_kind_id]
}
