use std::ops::{Add, Sub, Mul, Div, Rem, Neg};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};

/// A 2D vector, generally used with
#[derive(Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64
}

pub fn vec2<T: Into<f64>, U: Into<f64>>(x: T, y: U) -> Vec2 {
    Vec2::new(x, y)
}

impl Vec2 {
    pub fn new<T: Into<f64>, U: Into<f64>>(x: T, y: U) -> Vec2 {
        Vec2 { x: x.into(), y: y.into() }
    }

    /// The squared length of this vector
    pub fn length_sq(self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    /// The length of this vector
    pub fn length(self) -> f64 {
        self.length_sq().sqrt()
    }

    /// The normalized version of this vector
    pub fn normalized(self) -> Vec2 {
        self / self.length()
    }
}

//region Operators

// Vector-vector operations
impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec2 {
            x: -self.x,
            y: -self.y
        }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl Div for Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }
}

impl Rem for Vec2 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x % rhs.x,
            y: self.y % rhs.y
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl RemAssign for Vec2 {
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
    }
}

// Vector-number operations

impl<T: Into<f64>> Mul<T> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        let rhs = rhs.into();
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

// impl<T: Into<f64>> Mul<Vec2> for T {
//     type Output = Self;
//
//     fn mul(self, rhs: Vec2) -> Self {
//         let lhs = self.into();
//         Vec2 {
//             x: rhs.x * lhs,
//             y: rhs.y * lhs
//         }
//     }
// }

impl<T: Into<f64>> Div<T> for Vec2 {
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        let rhs = rhs.into();
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

// impl<T: Into<f64>> Div<Vec2> for T {
//     type Output = Self;
//
//     fn div(self, rhs: Vec2) -> Self {
//         let lhs = self.into();
//         Vec2 {
//             x: rhs.x / lhs,
//             y: rhs.y / lhs
//         }
//     }
// }

impl<T: Into<f64>> Rem<T> for Vec2 {
    type Output = Self;

    fn rem(self, rhs: T) -> Self {
        let rhs = rhs.into();
        Vec2 {
            x: self.x % rhs,
            y: self.y % rhs
        }
    }
}

// impl<T: Into<f64>> Rem<Vec2> for T {
//     type Output = Self;
//
//     fn rem(self, rhs: Vec2) -> Self {
//         let lhs = self.into();
//         Vec2 {
//             x: rhs.x % lhs,
//             y: rhs.y % lhs
//         }
//     }
// }

impl<T: Into<f64>> MulAssign<T> for Vec2 {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Into<f64>> DivAssign<T> for Vec2 {
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T: Into<f64>> RemAssign<T> for Vec2 {
    fn rem_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.x %= rhs;
        self.y %= rhs;
    }
}

//endregion
