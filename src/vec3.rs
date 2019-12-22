use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }
}

impl<T: Hash> Hash for Vec3<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

impl<T: Eq> Eq for Vec3<T> {}

impl<T: Default> Default for Vec3<T> {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }
    }
}

use std::fmt::Display;
impl<T: Display> Display for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;
    fn add(self, o: Self) -> Self::Output {
        Self {
            x: self.x + o.x,
            y: self.y + o.y,
            z: self.z + o.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;
    fn sub(self, o: Self) -> Self::Output {
        Self {
            x: self.x - o.x,
            y: self.y - o.y,
            z: self.z - o.z,
        }
    }
}

impl<T: Mul<Output = T>> Mul for Vec3<T> {
    type Output = Self;
    fn mul(self, o: Self) -> Self::Output {
        Self {
            x: self.x * o.x,
            y: self.y * o.y,
            z: self.z * o.z,
        }
    }
}

impl<T: Div<Output = T>> Div for Vec3<T> {
    type Output = Self;
    fn div(self, o: Self) -> Self::Output {
        Self {
            x: self.x / o.x,
            y: self.y / o.y,
            z: self.z / o.z,
        }
    }
}

impl<T: Add<Output = T> + Copy> AddAssign for Vec3<T> {
    fn add_assign(&mut self, o: Self) {
        *self = Self {
            x: self.x + o.x,
            y: self.y + o.y,
            z: self.z + o.z,
        };
    }
}

impl<T: Sub<Output = T> + Copy> SubAssign for Vec3<T> {
    fn sub_assign(&mut self, o: Self) {
        *self = Self {
            x: self.x - o.x,
            y: self.y - o.y,
            z: self.z - o.z,
        };
    }
}

impl<T: Mul<Output = T> + Copy> MulAssign for Vec3<T> {
    fn mul_assign(&mut self, o: Self) {
        *self = Self {
            x: self.x * o.x,
            y: self.y * o.y,
            z: self.z * o.z,
        };
    }
}

impl<T: Div<Output = T> + Copy> DivAssign for Vec3<T> {
    fn div_assign(&mut self, o: Self) {
        *self = Self {
            x: self.x / o.x,
            y: self.y / o.y,
            z: self.z / o.z,
        };
    }
}
