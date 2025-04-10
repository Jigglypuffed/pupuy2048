#[allow(long_running_const_eval)]
pub static MOVE_TABLE: [u32; 1 << 20] = {
    let mut moves = [0; 1 << 20];
    let mut i = 0;
    while i < moves.len() {
        let mut line = [
            (i as u32 >> 0) & 0x1f,
            (i as u32 >> 5) & 0x1f,
            (i as u32 >> 10) & 0x1f,
            (i as u32 >> 15) & 0x1f,
        ];

        let mut merged = false;
        let mut from = 3;
        let mut to = 3;
        loop {
            if line[from] == 0 {
                if from == 0 {
                    break;
                } else {
                    from -= 1;
                    continue;
                }
            }

            if !merged && to < 3 && line[from] == line[to + 1] {
                line[to + 1] += 1;
                line[from] = 0;
                merged = true;
            } else if to == from {
                if from == 0 {
                    break;
                } else {
                    to -= 1;
                }
            } else {
                line[to] = line[from];
                line[from] = 0;
                merged = false;
                to -= 1;
            }

            if from == 0 {
                break;
            } else {
                from -= 1;
            }
        }

        moves[i] = (line[0] << 0) | (line[1] << 5) | (line[2] << 10) | (line[3] << 15);

        i += 1;
    }
    moves
};
