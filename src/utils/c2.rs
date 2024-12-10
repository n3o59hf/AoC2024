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
    pub x: i32,
    pub y: i32,
}

impl C2 {
    pub const ZERO: C2 = C2 { x: 0, y: 0 };
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

impl C2 {
    pub fn rotate_right(&self) -> Self {
        C2 {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn mirror(self, around: C2) -> C2 {
        let delta = around - self;
        around + delta
    }

    pub fn neighbors_4(&self) -> [C2; 4] {
        [
            C2::new(self.x, self.y - 1),
            C2::new(self.x + 1, self.y),
            C2::new(self.x, self.y + 1),
            C2::new(self.x - 1, self.y),
        ]
    }
}
