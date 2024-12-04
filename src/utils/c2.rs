use std::fmt::Display;
use std::ops::{Add, Mul, Sub};

pub static C2_8_NEIGHBORS: [C2; 8] = [
    C2::new(0, -1),
    C2::new(1, -1),
    C2::new(1, 0),
    C2::new(1, 1),
    C2::new(0, 1),
    C2::new(-1, 1),
    C2::new(-1, 0),
    C2::new(-1, -1),
];

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct C2 {
    x: i32,
    y: i32,
}

impl C2 {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Display for C2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

impl Add for C2 {
    type Output = Self; // The resulting type after addition

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Implementing the Sub trait
impl Sub for C2 {
    type Output = Self; // The resulting type after subtraction

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i32> for C2 {
    type Output = Self;

    fn mul(self, scalar: i32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Mul<C2> for i32 {
    type Output = C2;

    fn mul(self, c: C2) -> C2 {
        C2 {
            x: c.x * self,
            y: c.y * self,
        }
    }
}
