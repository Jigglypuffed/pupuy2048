use move_lookup::MOVE_TABLE;

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

    pub fn add_n(&mut self, sq: u32, n: u32) -> bool {
        let target = &mut self.squares[sq as usize / 4][sq as usize % 4];
        if *target == 0 {
            *target = n;
            true
        } else {
            false
        }
    }

    pub fn move_left(&mut self) -> bool {
        let mut legal = false;
        for y in 0..4 {
            let mut v = 0;
            for x in 0..4 {
                v = v * 32 + self.squares[y][x];
            }
            let mut c = MOVE_TABLE[v as usize].clone();
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
            let mut c = MOVE_TABLE[v as usize].clone();
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
            let mut c = MOVE_TABLE[v as usize].clone();
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
            let mut c = MOVE_TABLE[v as usize].clone();
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
            a.add_n(12, 3); a.squares
        },
        [[2,  5,  8, 10],
         [3,  0,  0, 5],
         [10, 11, 1, 0],
         [3,  0,  0, 0]]
    );
    assert_eq!(
        {
            let mut a = Board::from_fen("258a/3005/ab10/0000");
            a.add_n(10, 3)
        },
        false
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

    let mut a = start.clone();
    let mut b = start.clone();
    let mut c = start.clone();
    let mut d = start.clone();

    assert_eq!(a.move_left(), true);
    assert_eq!(a.squares, moved_left.squares);

    assert_eq!(b.move_right(), true);
    assert_eq!(b.squares, moved_right.squares);

    assert_eq!(c.move_up(), true);
    assert_eq!(c.squares, moved_up.squares);

    assert_eq!(d.move_down(), true);
    assert_eq!(d.squares, moved_down.squares);
}
