use move_lookup::MOVE_TABLE;
use rand::Rng;

pub type Move = u8;
pub const MOVE_LEFT: Move = 1 << 0;
pub const MOVE_RIGHT: Move = 1 << 1;
pub const MOVE_UP: Move = 1 << 2;
pub const MOVE_DOWN: Move = 1 << 3;

#[derive(Debug, PartialEq, Eq)]
pub enum GameState {
    OnGoing,
    Lost,
}

#[derive(Debug, Clone, Copy)]
pub struct Board {
    squares: [[u32; 4]; 4],
}

#[allow(dead_code)]
impl Board {
    pub fn new(squares: [[u32; 4]; 4]) -> Self {
        Self { squares }
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut squares = [[0; 4]; 4];
        let mut x = 0;
        let mut y = 0;
        for c in fen.chars() {
            match c {
                '0' => squares[y][x] = 0,
                '1' => squares[y][x] = 1,
                '2' => squares[y][x] = 2,
                '3' => squares[y][x] = 3,
                '4' => squares[y][x] = 4,
                '5' => squares[y][x] = 5,
                '6' => squares[y][x] = 6,
                '7' => squares[y][x] = 7,
                '8' => squares[y][x] = 8,
                '9' => squares[y][x] = 9,
                'a' => squares[y][x] = 10,
                'b' => squares[y][x] = 11,
                'c' => squares[y][x] = 12,
                'd' => squares[y][x] = 13,
                'e' => squares[y][x] = 14,
                'f' => squares[y][x] = 15,
                '/' => {
                    y += 1;
                    x = 0
                }
                _ => panic!(),
            }
            if c != '/' {
                x += 1;
            }
        }
        Self { squares }
    }

    pub fn add_tile(&mut self, sq: u32, n: u32) -> bool {
        let target = &mut self.squares[sq as usize / 4][sq as usize % 4];
        if *target == 0 {
            *target = n;
            true
        } else {
            false
        }
    }

    pub fn add_random_tile(&mut self) {
        let mut empty: Vec<(usize, usize)> = vec![];

        for y in 0..4 {
            for x in 0..4 {
                if self.squares[y][x] == 0 {
                    empty.push((y, x));
                }
            }
        }
        let (y, x) = empty[rand::thread_rng().gen_range(0..empty.len())];
        let tile = if rand::random::<f32>() < 0.9 { 1 } else { 2 };
        self.squares[y][x] = tile;
    }

    pub fn move_left(&mut self) -> bool {
        let mut legal = false;
        for y in 0..4 {
            let mut v = 0;
            for x in 0..4 {
                v = v * 32 + self.squares[y][x];
            }
            let mut c = MOVE_TABLE[v as usize];
            if c == v {
                continue;
            }
            legal = true;
            for x in (0..4).rev() {
                self.squares[y][x] = c % 32;
                c /= 32;
            }
        }

        legal
    }

    pub fn move_right(&mut self) -> bool {
        let mut legal = false;
        for y in 0..4 {
            let mut v = 0;
            for x in (0..4).rev() {
                v = v * 32 + self.squares[y][x];
            }
            let mut c = MOVE_TABLE[v as usize];
            if c == v {
                continue;
            }
            legal = true;
            for x in 0..4 {
                self.squares[y][x] = c % 32;
                c /= 32;
            }
        }

        legal
    }

    pub fn move_up(&mut self) -> bool {
        let mut legal = false;
        for x in 0..4 {
            let mut v = 0;
            for y in 0..4 {
                v = v * 32 + self.squares[y][x];
            }
            let mut c = MOVE_TABLE[v as usize];
            if c == v {
                continue;
            }
            legal = true;
            for y in (0..4).rev() {
                self.squares[y][x] = c % 32;
                c /= 32;
            }
        }

        legal
    }

    pub fn move_down(&mut self) -> bool {
        let mut legal = false;
        for x in 0..4 {
            let mut v = 0;
            for y in (0..4).rev() {
                v = v * 32 + self.squares[y][x];
            }
            let mut c = MOVE_TABLE[v as usize];
            if c == v {
                continue;
            }
            legal = true;
            for y in 0..4 {
                self.squares[y][x] = c % 32;
                c /= 32;
            }
        }

        legal
    }

    pub fn apply_move(&mut self, mv: Move) -> bool {
        match mv {
            MOVE_LEFT => self.move_left(),
            MOVE_RIGHT => self.move_right(),
            MOVE_UP => self.move_up(),
            MOVE_DOWN => self.move_down(),
            _ => panic!("Unexpected move format"),
        }
    }

    pub fn get_legal_moves(&self) -> Move {
        (self.clone().move_left() as Move * MOVE_LEFT)
            | (self.clone().move_right() as Move * MOVE_RIGHT)
            | (self.clone().move_up() as Move * MOVE_UP)
            | (self.clone().move_down() as Move * MOVE_DOWN)
    }

    pub fn get_game_state(&self) -> GameState {
        if self.get_legal_moves() > 0 {
            GameState::OnGoing
        } else {
            GameState::Lost
        }
    }

    pub fn evaluate(&self) -> f32 {
        let mut empty = 0;
        let mut sum = 0.0;

        for y in 0..4 {
            for x in 0..4 {
                let v = self.squares[y][x];
                if v == 0 {
                    empty += 1;
                } else {
                    sum += 2f32.powi(v as i32);
                }
            }
        }

        sum + empty as f32 * 10.0 // weight for empty cells
    }
}

#[rustfmt::skip]
#[test]
fn fen_parse_test() {
    assert_eq!(
        Board::from_fen("258a/3005/ab10/0000").squares,
        [[2,  5,  8, 10],
         [3,  0,  0, 5],
         [10, 11, 1, 0],
         [0,  0,  0, 0]]
    )
}

#[rustfmt::skip]
#[test]
fn add_n_test() {
    assert_eq!(
        {
            let mut a = Board::from_fen("258a/3005/ab10/0000");
            a.add_tile(12, 3); a.squares
        },
        [[2,  5,  8, 10],
         [3,  0,  0, 5],
         [10, 11, 1, 0],
         [3,  0,  0, 0]]
    );
    assert!(
        !{
            let mut a = Board::from_fen("258a/3005/ab10/0000");
            a.add_tile(10, 3)
        }
    )
}

#[rustfmt::skip]
#[test]
fn move_test() {
    let start = Board::new(
        [[0, 0, 0, 1],
         [0, 2, 0, 2],
         [2, 2, 2, 2],
         [3, 1, 1, 1]]);
    let moved_left = Board::new(
        [[1, 0, 0, 0],
         [3, 0, 0, 0],
         [3, 3, 0, 0],
         [3, 2, 1, 0]]);
    let moved_right = Board::new(
        [[0, 0, 0, 1],
         [0, 0, 0, 3],
         [0, 0, 3, 3],
         [0, 3, 1, 2]]);
    let moved_up = Board::new(
        [[2, 3, 2, 1],
         [3, 1, 1, 3],
         [0, 0, 0, 1],
         [0, 0, 0, 0]]);
    let moved_down = Board::new(
        [[0, 0, 0, 0],
         [0, 0, 0, 1],
         [2, 3, 2, 3],
         [3, 1, 1, 1]]);

    assert_eq!(start.get_legal_moves(), 15);

    let mut a = start;
    let mut b = start;
    let mut c = start;
    let mut d = start;

    assert!(a.move_left());
    assert_eq!(a.squares, moved_left.squares);

    assert!(b.move_right());
    assert_eq!(b.squares, moved_right.squares);

    assert!(c.move_up());
    assert_eq!(c.squares, moved_up.squares);

    assert!(d.move_down());
    assert_eq!(d.squares, moved_down.squares);
}
