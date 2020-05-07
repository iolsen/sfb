// https://www.redblobgames.com/grids/hexagons
// The game map uses offset coordinates in an "even-q" layout.

/* The hex facing printed at the bottom left of the map.
 * A is straight up, clockwise from there
 *
 *    A
 *  F   B
 *  E   C
 *    D
 */
pub enum Facing {
    A,
    B,
    C,
    D,
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

#[derive(Eq, Debug)]
pub struct Hex {
    pub col: i8,
    pub row: i8,

    _private: ()
}

impl PartialEq for Hex {
    fn eq(&self, other: &Self) -> bool {
        return self.col == other.col && self.row == other.row;
    }
}

struct Cube {
    x: i8,
    y: i8,
    z: i8
}

impl Hex {
    pub fn new(col: i8, row: i8) -> Hex {
        if col < MIN_COL || col > MAX_COL || row < MIN_ROW || row > MAX_ROW {
            return OOB_HEX;
        }
        Hex { col, row, _private: () }
    }

    pub fn number(&self) -> String {
        let n = self.col as i16 * 100 + self.row as i16;
        return format!{"{:04}", n}
    }

    pub fn neighbor(&self, f: Facing) -> Hex {
        if self.is_oob() {
            return OOB_HEX;
        }
        let parity = (self.col & 1) as usize;
        let dir = DIRECTIONS[parity][f as usize];
        return Hex::new((self.col + dir[0]) as i8, (self.row + dir[1]) as i8);
    }

    pub fn distance_to(&self, other: Hex) -> i8 {
        return Hex::cubic_distance(self.to_cube(), other.to_cube());
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

    fn to_cube(&self) -> Cube {
        let x = self.col;
        let z = self.row - (self.col + (self.col & 1)) / 2;
        let y = -x-z;
        return Cube {x, y, z}
    }

    fn _from_cube(cube: Cube) -> Hex {
        Hex::new(cube.x, cube.z + (cube.x + (cube.x & 1)) / 2)
    }

    fn cubic_distance(a: Cube, b: Cube) -> i8 {
        return ((a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()) / 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_bounds() {
        let h0101 = Hex::new(1, 1);
        assert_eq!(h0101.number(), "0101");

        let h6030 = Hex::new(60, 30);
        assert_eq!(h6030.number(), "6030");

        assert_eq!(OOB_HEX.number(), "0000");
    }

    #[test]
    fn oob_neighbor_is_oob() {
        assert!(OOB_HEX.neighbor(Facing::A).is_oob());
    }

    #[test]
    fn upper_left_boundary_neighbors() {
        let h = Hex::new(MIN_COL, MIN_ROW);
        let n = h.neighbor(Facing::A);
        assert!(n.is_oob());
        let n = h.neighbor(Facing::B);
        assert!(n.is_oob());
        let n = h.neighbor(Facing::C);
        assert_eq!(n, Hex::new(MIN_COL+1, MIN_ROW));
        let n = h.neighbor(Facing::D);
        assert_eq!(n, Hex::new(MIN_COL, MIN_ROW+1));
        let n = h.neighbor(Facing::E);
        assert!(n.is_oob());
        let n = h.neighbor(Facing::F);
        assert!(n.is_oob());
    }

    #[test]
    fn lower_right_boundary_neighbors() {
        let h = Hex::new(MAX_COL, MAX_ROW);
        let n = h.neighbor(Facing::A);
        assert_eq!(n, Hex::new(MAX_COL, MAX_ROW-1));
        let n = h.neighbor(Facing::B);
        assert!(n.is_oob());
        let n = h.neighbor(Facing::C);
        assert!(n.is_oob());
        let n = h.neighbor(Facing::D);
        assert!(n.is_oob());
        let n = h.neighbor(Facing::E);
        assert!(n.is_oob());
        let n = h.neighbor(Facing::F);
        assert_eq!(n, Hex::new(MAX_COL-1, MAX_ROW));
    }

    #[test]
    fn distance_sanity() {
        assert_eq!(0, Hex::new(1, 1).distance_to(Hex::new(1, 1)));
        assert_eq!(1, Hex::new(1, 1).distance_to(Hex::new(2, 1)));
        assert_eq!(1, Hex::new(1, 1).distance_to(Hex::new(1, 2)));
        assert_eq!(9, Hex::new(1, 1).distance_to(Hex::new(1, 10)));
        assert_eq!(9, Hex::new(1, 1).distance_to(Hex::new(10, 1)));
    }
}
