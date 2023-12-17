use std::ops::Mul;

#[derive(Clone, Copy)]
pub struct Direction {
    pub x: i32,
    pub y: i32,
}

impl Direction {
    pub fn invert(&self) -> Direction {
        Direction {
            x: -self.x,
            y: -self.y,
        }
    }
}

pub static DIRECTIONS: [Direction; 3] = [
    Direction { x: 1, y: 0 },
    Direction { x: 0, y: 1 },
    Direction { x: 1, y: 1 },
];

impl Mul<i32> for Direction {
    type Output = Direction;

    fn mul(self, rhs: i32) -> Self::Output {
        Direction {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}
