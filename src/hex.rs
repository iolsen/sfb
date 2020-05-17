// https://www.redblobgames.com/grids/hexagons
// The game map uses offset coordinates in an "even-q" layout.

use std::fmt;

/* The hex facing printed at the bottom left of the map.
 * A is straight up, clockwise from there
 *
 *    A
 *  F   B
 *  E   C
 *    D
 */
#[derive(PartialEq, Eq, Debug)]
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

#[derive(PartialEq, Eq, Debug)]
struct Cube {
    x: i8,
    y: i8,
    z: i8
}

impl fmt::Display for Hex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}", self.number())
    }
}

impl Hex {
    pub fn new(col: i8, row: i8) -> Option<Hex> {
        if col < MIN_COL || col > MAX_COL || row < MIN_ROW || row > MAX_ROW {
            return None;
        }
        Some(Hex { col, row, _private: () })
    }

    pub fn number(&self) -> i16 {
        self.col as i16 * 100 + self.row as i16
    }

    pub fn neighbor(&self, f: Facing) -> Option<Hex> {
        let parity = (self.col & 1) as usize;
        let dir = DIRECTIONS[parity][f as usize];
        Hex::new((self.col + dir[0]) as i8, (self.row + dir[1]) as i8)
    }

    pub fn distance_to(&self, other: &Hex) -> i8 {
        return Hex::cubic_distance(self.to_cube(), other.to_cube());
    }

    pub fn shield_facing_from(&self, other: &Hex) -> Facing {
        let theta = self.angle_to(other);
        if theta >= 60 && theta < 120 { return Facing::A }
        if theta >= 0 && theta < 60 { return Facing::B }
        if theta >= 300 && theta < 360 { return Facing::C }
        if theta >= 240 && theta < 300 { return Facing::D }
        if theta >= 180 && theta < 240 { return Facing::E }
        Facing::F
    }

    fn angle_to(&self, other: &Hex) -> i16 {
        let self_center = self.center_coords();
        let other_center = other.center_coords();

        let dx = other_center.0 - self_center.0;
        let dy = self_center.1 - other_center.1;

        if dx == 0_f64 {
            if dy >= 0_f64 { return 90 }
            return 270;
        }
        let t = (dy/dx).atan().to_degrees();
        if t >= 0_f64 {
            if dx > 0_f64 {
                return (t + 0.5) as i16
            } else {
                return (t + 180_f64 + 0.5) as i16;
            }
        } else {
            if dx > 0_f64 {
                return (t + 360_f64 + 0.5) as i16;
            } else {
                return (t + 180_f64 + 0.5) as i16;
            }
        }
    }

    fn center_coords(&self) -> (f64, f64) {
       let tx = self.number()/100;
       let sqrt_3 = 3_f64.sqrt();
       let x = 1_f64/(2_f64*sqrt_3) + (tx-1) as f64 * (sqrt_3/2_f64);

       let ty = (self.number() % 100) as f64;
       let y = ty - 0.5*(tx % 2) as f64;

       (x, y)
    }

    fn to_cube(&self) -> Cube {
        let x = self.col;
        let z = self.row - (self.col + (self.col & 1)) / 2;
        let y = -x-z;
        return Cube {x, y, z}
    }

    fn _from_cube(cube: Cube) -> Option<Hex> {
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
        match h0101 {
            Some(h) => assert_eq!("0101", format!("{}", h)),
            None => assert!(false)
        }

        let h6030 = Hex::new(60, 30);
        match h6030 {
            Some(h) => assert_eq!("6030", format!("{}", h)),
            None => assert!(false)
        }
    }

    #[test]
    fn display_formatter() {
        let h0101 = Hex::new(1, 1);
        match h0101 {
            Some(h) => assert_eq!(h.number(), 101),
            None => assert!(false)
        }

        let h6030 = Hex::new(60, 30);
        match h6030 {
            Some(h) => assert_eq!(h.number(), 6030),
            None => assert!(false)
        }
    }

    #[test]
    fn upper_left_boundary_neighbors() {
        if let Some(h) = Hex::new(MIN_COL, MIN_ROW) {
            assert!(h.neighbor(Facing::A).is_none());
            assert!(h.neighbor(Facing::B).is_none());
            assert_eq!(h.neighbor(Facing::C), Hex::new(MIN_COL+1, MIN_ROW));
            assert_eq!(h.neighbor(Facing::D), Hex::new(MIN_COL, MIN_ROW+1));
            assert!(h.neighbor(Facing::E).is_none());
            assert!(h.neighbor(Facing::F).is_none());
        }
        else { assert!(false) }
    }

    #[test]
    fn lower_right_boundary_neighbors() {
        if let Some(h) = Hex::new(MAX_COL, MAX_ROW) {
            assert_eq!(h.neighbor(Facing::A), Hex::new(MAX_COL, MAX_ROW-1));
            assert!(h.neighbor(Facing::B).is_none());
            assert!(h.neighbor(Facing::C).is_none());
            assert!(h.neighbor(Facing::D).is_none());
            assert!(h.neighbor(Facing::E).is_none());
            assert_eq!(h.neighbor(Facing::F), Hex::new(MAX_COL-1, MAX_ROW));
        }
        else { assert!(false) }
    }

    #[test]
    fn distance_sanity() {
        if let Some(h) = Hex::new(1, 1) {
            assert_eq!(0, h.distance_to(&h));

            if let Some(h2) = Hex::new(2, 1) { assert_eq!(1, h.distance_to(&h2)) }
            else { assert!(false) }

            if let Some(h2) = Hex::new(1, 2) { assert_eq!(1, h.distance_to(&h2)) }
            else { assert!(false) }

            if let Some(h2) = Hex::new(1, 10) { assert_eq!(9, h.distance_to(&h2)) }
            else { assert!(false) }

            if let Some(h2) = Hex::new(10, 1) { assert_eq!(9, h.distance_to(&h2)) }
            else { assert!(false) }
        }
        else { assert!(false) }
    }

    #[test]
    fn shield_facing_from_sanity() {
        if let Some(h) = Hex::new(40, 2) {
            if let Some(h2) = Hex::new(40, 1) { assert_eq!(Facing::A, h.shield_facing_from(&h2)) }
            else { assert!(false) }
            if let Some(h2) = Hex::new(41, 2) { assert_eq!(Facing::B, h.shield_facing_from(&h2)) }
            else { assert!(false) }
            if let Some(h2) = Hex::new(41, 3) { assert_eq!(Facing::C, h.shield_facing_from(&h2)) }
            else { assert!(false) }
            if let Some(h2) = Hex::new(40, 3) { assert_eq!(Facing::D, h.shield_facing_from(&h2)) }
            else { assert!(false) }
            if let Some(h2) = Hex::new(39, 3) { assert_eq!(Facing::E, h.shield_facing_from(&h2)) }
            else { assert!(false) }
            if let Some(h2) = Hex::new(39, 2) { assert_eq!(Facing::F, h.shield_facing_from(&h2)) }
            else { assert!(false) }

            if let Some(h2) = Hex::new(39, 7) { assert_eq!(Facing::D, h.shield_facing_from(&h2)) }
            else { assert!(false) }
        }
        else { assert!(false) }
    }
}
