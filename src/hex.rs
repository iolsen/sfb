// https://www.redblobgames.com/grids/hexagons
// The game map uses offset coordinates in an "even-q" layout.
pub struct Hex {
    pub col: i8,
    pub row: i8,

    _private: ()
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

const MIN_COL: i8 = 1;
const MAX_COL: i8 = 60;
const MIN_ROW: i8 = 1;
const MAX_ROW: i8 = 30;
const OOB_HEX: Hex = Hex { col: 0, row: 0, _private: () };
const DIRECTIONS: [[[i8; 2]; 6]; 2] = [
    [[0,-1], [1,0], [1,1], [0,1], [-1,1], [-1,0]],
    [[0,-1], [1,-1], [1,0], [0,1], [-1,0], [-1,-1]]
];

impl Hex {
    pub fn new(col: i8, row: i8) -> Hex {
        if col < MIN_COL || col > MAX_COL || row < MIN_ROW || row > MAX_ROW {
            return OOB_HEX;
        }
        Hex { col, row, _private: () }
    }

    pub fn number(&self) -> i16 {
        return self.col as i16 * 100 + self.row as i16;
    }

    pub fn neighbor(&self, d: Direction) -> Hex {
        if self.is_oob() {
            return OOB_HEX;
        }
        let parity = (self.col & 1) as usize;
        let dir = DIRECTIONS[parity][d as usize];
        return Hex::new((self.col + dir[0]) as i8, (self.row + dir[1]) as i8);
    }

    fn is_oob(&self) -> bool {
        if self.col < MIN_COL || self.row < MIN_ROW {
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
        let h0101 = Hex::new(1, 1);
        assert_eq!(h0101.number(), 101);

        let h6030 = Hex::new(60, 30);
        assert_eq!(h6030.number(), 6030);
    }

    #[test]
    fn oob_neighbor_is_oob() {
        assert!(OOB_HEX.neighbor(Direction::A).is_oob());
    }

    #[test]
    fn upper_left_boundary_neighbors() {
        let h = Hex::new(MIN_COL, MIN_ROW);
        let n = h.neighbor(Direction::A);
        assert!(n.is_oob());
        let n = h.neighbor(Direction::B);
        assert!(n.is_oob());
        let n = h.neighbor(Direction::C);
        assert_eq!(n.number(), Hex::new(MIN_COL+1, MIN_ROW).number());
        let n = h.neighbor(Direction::D);
        assert_eq!(n.number(), Hex::new(MIN_COL, MIN_ROW+1).number());
        let n = h.neighbor(Direction::E);
        assert!(n.is_oob());
        let n = h.neighbor(Direction::F);
        assert!(n.is_oob());
    }

    #[test]
    fn lower_right_boundary_neighbors() {
        let h = Hex::new(MAX_COL, MAX_ROW);
        let n = h.neighbor(Direction::A);
        assert_eq!(n.number(), Hex::new(MAX_COL, MAX_ROW-1).number());
        let n = h.neighbor(Direction::B);
        assert!(n.is_oob());
        let n = h.neighbor(Direction::C);
        assert!(n.is_oob());
        let n = h.neighbor(Direction::D);
        assert!(n.is_oob());
        let n = h.neighbor(Direction::E);
        assert!(n.is_oob());
        let n = h.neighbor(Direction::F);
        assert_eq!(n.number(), Hex::new(MAX_COL-1, MAX_ROW).number());
    }
}


