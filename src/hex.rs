// https://www.redblobgames.com/grids/hexagons
// The game map uses offset coordinates in an "even-q" layout.
pub struct Hex {
    col: i8,
    row: i8
}

/* The directions printed at the bottom left of the map.
 * A is straight up, clockwise from there */
pub enum Direction {
    A, // up
    B,
    C,
    D, // down
    E,
    F
}

const MAX_COL: i8 = 60;
const MAX_ROW: i8 = 30;
pub const OOB_HEX: Hex = Hex { col: 0, row: 0 };

impl Hex {
    pub fn number(&self) -> i16 {
        return self.col as i16 * 100 + self.row as i16;
    }

    pub fn neighbor(&self, d: Direction) -> Hex {
        let directions = [
            [[0,-1], [1,0], [1,1], [0,1], [-1,1], [-1,0]],
            [[0,-1], [1,-1], [1,0], [0,1], [-1,0], [-1,-1]]
        ];
        let parity = (self.col & 1) as usize;
        let dir = directions[parity][d as usize];
        let n = Hex { col: (self.col + dir[0]) as i8, row: (self.row + dir[1]) as i8 };
        if n.is_oob() {
           return OOB_HEX;
        }
        return n;
    }

    fn is_oob(&self) -> bool {
        if self.col < 1 || self.row < 1 {
            return true;
        }
        if self.col > MAX_COL {
            return true;
        }
        return self.row > MAX_ROW;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_bounds() {
        let h0101 = Hex { row: 1, col: 1 };
        assert_eq!(h0101.number(), 101);

        let h6131 = Hex { row: 31, col: 61 };
        assert_eq!(h6131.number(), 6131);
    }

    #[test]
    fn neighbors() {
        let h0101 = Hex { row: 1, col: 1 };
        let n = h0101.neighbor(Direction::A);
        assert!(n.is_oob());
        let n = h0101.neighbor(Direction::B);
        assert!(n.is_oob());
        let n = h0101.neighbor(Direction::C);
        assert_eq!(n.number(), 201);
        let n = h0101.neighbor(Direction::D);
        assert_eq!(n.number(), 102);
        let n = h0101.neighbor(Direction::E);
        assert!(n.is_oob());
        let n = h0101.neighbor(Direction::F);
        assert!(n.is_oob());
    }
}


