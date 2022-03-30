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

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn tetriminos(kind: TetriminoKind, direction: Direction) -> [[usize; 2]; 4] {
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
