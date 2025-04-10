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

    pub fn move_left(&mut self) -> bool {
        let mut legal = false;
        for x in 0..4 {
            let mut v = 0;
            for y in 0..4 {
                v = v * 32 + self.squares[x][y];
            }
            let mut c = MOVE_TABLE[v as usize].clone();
            if c == v {
                continue;
            }
            legal = true;
            for y in (0..4).rev() {
                self.squares[x][y] = c % 32;
                c /= 32;
            }
        }

        legal
    }

    pub fn move_right(&mut self) -> bool {
        let mut legal = false;
        for x in 0..4 {
            let mut v = 0;
            for y in (0..4).rev() {
                v = v * 32 + self.squares[x][y];
            }
            let mut c = MOVE_TABLE[v as usize].clone();
            if c == v {
                continue;
            }
            legal = true;
            for y in 0..4 {
                self.squares[x][y] = c % 32;
                c /= 32;
            }
        }

        legal
    }

    pub fn move_up(&mut self) -> bool {
        let mut legal = false;
        for y in 0..4 {
            let mut v = 0;
            for x in 0..4 {
                v = v * 32 + self.squares[x][y];
            }
            let mut c = MOVE_TABLE[v as usize].clone();
            if c == v {
                continue;
            }
            legal = true;
            for x in (0..4).rev() {
                self.squares[x][y] = c % 32;
                c /= 32;
            }
        }

        legal
    }

    pub fn move_down(&mut self) -> bool {
        let mut legal = false;
        for y in 0..4 {
            let mut v = 0;
            for x in (0..4).rev() {
                v = v * 32 + self.squares[x][y];
            }
            let mut c = MOVE_TABLE[v as usize].clone();
            if c == v {
                continue;
            }
            legal = true;
            for x in 0..4 {
                self.squares[x][y] = c % 32;
                c /= 32;
            }
        }

        legal
    }
}

#[test]
#[rustfmt::skip]
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
