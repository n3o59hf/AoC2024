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

pub const C2_UP: C2 = C2::new(0, -1);
pub const C2_DOWN: C2 = C2::new(0, 1);
pub const C2_LEFT: C2 = C2::new(-1, 0);
pub const C2_RIGHT: C2 = C2::new(1, 0);

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
    #[inline]
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
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i32> for C2 {
    type Output = Self;
    #[inline]
    fn mul(self, scalar: i32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Mul<C2> for i32 {
    type Output = C2;
    #[inline]
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

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct C2Field<T> {
    width: usize,
    height: usize,
    store: Vec<T>,
    indices: Vec<C2>,
}

#[allow(dead_code)]
impl<T> C2Field<T>
where
    T: Clone + Default,
{
    fn indices(width: usize, height: usize) -> Vec<C2> {
        let mut indices = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                indices.push(C2::new(x as i32, y as i32));
            }
        }
        indices
    }

    #[allow(dead_code)]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            store: vec![T::default(); width * height],
            indices: Self::indices(width, height),
        }
    }

    pub fn from_string(input: &str, mapping: fn(char) -> T) -> C2Field<T> {
        let mut store = Vec::new();
        let width = input.find("\n").expect("At least one endline");
        let lines = input.lines();
        let mut height = 0;

        for line in lines {
            height += 1;
            let line = line.chars().map(mapping);
            store.extend(line);
            if store.len() != width * height {
                panic!("Not all lines are the same")
            }
        }

        C2Field {
            width,
            height,
            store,
            indices: Self::indices(width, height),
        }
    }

    #[inline(always)]
    pub fn indice(&self, coord: &C2) -> usize {
        coord.y as usize * self.width + coord.x as usize
    }

    pub fn get(&self, coord: &C2) -> Option<&T> {
        if coord.x < 0
            || coord.y < 0
            || coord.y as usize >= self.height
            || coord.x as usize >= self.width
        {
            None
        } else {
            Some(&self.store[self.indice(coord)])
        }
    }

    pub fn set(&mut self, coord: &C2, item: T) {
        debug_assert!(
            !(coord.x < 0
                || coord.y < 0
                || coord.y as usize >= self.height
                || coord.x as usize >= self.width)
        );
        let i = self.indice(coord);
        self.store[i] = item;
    }
    pub fn iter(&self) -> impl Iterator<Item = (&C2, &T)> {
        self.indices.iter().zip(self.store.iter())
    }
    #[inline]
    pub fn keys(&self) -> &Vec<C2> {
        &self.indices
    }

    #[inline]
    pub fn values(&self) -> &Vec<T> {
        &self.store
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }
}
