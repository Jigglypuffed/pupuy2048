#![allow(long_running_const_eval)]
pub static MOVE_TABLE: [u32; 1 << 20] = build_move_lut();

const fn build_move_lut() -> [u32; 1 << 20] {
    let mut line = [0u32; 4];
    let mut moves = [0; 1 << 20];
    let mut i = 0;
    while i < 1 << 20 {
        let mut j = 0;
        while j < 4 {
            line[j] = (i as u32 >> (j * 5)) & 0x1f;
            j += 1;
        }

        let mut merged = false;
        let mut new_x = 3;
        let mut x = 3;
        loop {
            if line[x] == 0 {
                if x == 0 {
                    break;
                } else {
                    x -= 1;
                    continue;
                }
            }

            if !merged && new_x < 3 && line[x] == line[new_x + 1] {
                line[new_x + 1] += 1;
                line[x] = 0;
                merged = true;
            } else if new_x == x {
                if x == 0 {
                    break;
                } else {
                    new_x -= 1;
                }
            } else {
                line[new_x] = line[x];
                line[x] = 0;
                merged = false;
                new_x -= 1;
            }

            if x == 0 {
                break;
            } else {
                x -= 1;
            }
        }

        moves[i] = 0;
        let mut j = 0;
        while j < 4 {
            moves[i] += line[j] << (j * 5);
            j += 1
        }

        i += 1;
    }
    moves
}
