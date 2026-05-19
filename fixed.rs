// fixed-point arithmetic for deterministic simulation

use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Fixed(i32);

impl Fixed {
    pub const ZERO: Fixed = Fixed(0);
    pub const ONE: Fixed = Fixed(1 << 16);
    pub const HALF: Fixed = Fixed(1 << 15);
    pub const MAX: Fixed = Fixed(i32::MAX);
    pub const MIN: Fixed = Fixed(i32::MIN);

    const SCALE: i32 = 1 << 16;
    const SCALE_I64: i64 = 1 << 16;

    // create from int
    #[inline]
    pub const fn from_int(n: i32) -> Self {
        Fixed(n << 16)
    }

    // create from bits
    #[inline]
    pub const fn from_bits(bits: i32) -> Self {
        Fixed(bits)
    }

    // access raw bits
    #[inline]
    pub const fn to_bits(self) -> i32 {
        self.0
    }

    // convert to f64 (for display/debugging purposes only)
    #[inline]
    pub fn to_f64_debug(self) -> f64 {
        self.0 as f64 / Self::SCALE as f64
    }

    // create from f64 constant (compile-time initialization only)
    pub fn from_f64_const(f: f64) -> Self {
        Fixed((f * Self::SCALE as f64).round() as i32)
    }

    /// abs val
    #[inline]
    pub fn abs(self) -> Self {
        Fixed(self.0.abs())
    }

    // saturating add against overflow
    #[inline]
    pub fn saturating_add(self, rhs: Fixed) -> Fixed {
        Fixed(self.0.saturating_add(rhs.0))
    }

    // int floor
    #[inline]
    pub fn floor(self) -> i32 {
        self.0 >> 16
    }

    // int ceiling
    #[inline]
    pub fn ceil(self) -> i32 {
        (self.0 + Self::SCALE - 1) >> 16
    }

    // square root
    pub fn sqrt(self) -> Fixed {
        if self.0 <= 0 {
            return Fixed::ZERO;
        }
        let n = (self.0 as i64) << 16;
        let mut x = (n as f64).sqrt() as i64; 
        // newton-raphson:
        for _ in 0..8 {
            x = (x + n / x) / 2;
        }
        Fixed(x as i32)
    }

    // approx int sqrt for distance comparisons
    pub fn squared(self) -> Fixed {
        Fixed(((self.0 as i64 * self.0 as i64) >> 16) as i32)
    }

    // linear interpolation 
    pub fn lerp(self, other: Fixed, t: Fixed) -> Fixed {
        self + (other - self) * t
    }

    // clamp to [min, max]
    #[inline]
    pub fn clamp(self, min: Fixed, max: Fixed) -> Fixed {
        if self < min { min } else if self > max { max } else { self }
    }
}

impl Add for Fixed {
    type Output = Fixed;
    #[inline]
    fn add(self, rhs: Fixed) -> Fixed {
        Fixed(self.0 + rhs.0)
    }
}

impl AddAssign for Fixed {
    #[inline]
    fn add_assign(&mut self, rhs: Fixed) {
        self.0 += rhs.0;
    }
}

impl Sub for Fixed {
    type Output = Fixed;
    #[inline]
    fn sub(self, rhs: Fixed) -> Fixed {
        Fixed(self.0 - rhs.0)
    }
}

impl SubAssign for Fixed {
    #[inline]
    fn sub_assign(&mut self, rhs: Fixed) {
        self.0 -= rhs.0;
    }
}

impl Mul for Fixed {
    type Output = Fixed;
    #[inline]
    fn mul(self, rhs: Fixed) -> Fixed {
        // promote to i64 to avoid overflow, then shift back
        Fixed(((self.0 as i64 * rhs.0 as i64) >> 16) as i32)
    }
}

impl Mul<i32> for Fixed {
    type Output = Fixed;
    #[inline]
    fn mul(self, rhs: i32) -> Fixed {
        Fixed(self.0 * rhs)
    }
}

impl Div for Fixed {
    type Output = Fixed;
    #[inline]
    fn div(self, rhs: Fixed) -> Fixed {
        Fixed(((self.0 as i64 * Self::SCALE_I64) / rhs.0 as i64) as i32)
    }
}

impl Neg for Fixed {
    type Output = Fixed;
    #[inline]
    fn neg(self) -> Fixed {
        Fixed(-self.0)
    }
}

impl fmt::Debug for Fixed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fixed({:.4})", self.to_f64_debug())
    }
}

impl fmt::Display for Fixed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.4}", self.to_f64_debug())
    }
}

// 2D point or vector in fixed-point space
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct Vec2 {
    pub x: Fixed,
    pub y: Fixed,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: Fixed::ZERO, y: Fixed::ZERO };

    pub fn new(x: Fixed, y: Fixed) -> Self {
        Vec2 { x, y }
    }

    pub fn from_ints(x: i32, y: i32) -> Self {
        Vec2 {
            x: Fixed::from_int(x),
            y: Fixed::from_int(y),
        }
    }

    // squared magnitude
    pub fn length_squared(self) -> Fixed {
        self.x * self.x + self.y * self.y
    }

    // euclidean magnitude
    pub fn length(self) -> Fixed {
        self.length_squared().sqrt()
    }

    // manhatten distance
    pub fn manhattan(self, other: Vec2) -> Fixed {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    // euclidian distance
    pub fn distance(self, other: Vec2) -> Fixed {
        (self - other).length()
    }

    // unit length or zero
    pub fn normalize(self) -> Vec2 {
        let len = self.length();
        if len == Fixed::ZERO {
            Vec2::ZERO
        } else {
            Vec2 { x: self.x / len, y: self.y / len }
        }
    }

    // dot product
    pub fn dot(self, other: Vec2) -> Fixed {
        self.x * other.x + self.y * other.y
    }

    // linear interpolation
    pub fn lerp(self, other: Vec2, t: Fixed) -> Vec2 {
        Vec2 {
            x: self.x.lerp(other.x, t),
            y: self.y.lerp(other.y, t),
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Mul<Fixed> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: Fixed) -> Vec2 {
        Vec2 { x: self.x * scalar, y: self.y * scalar }
    }
}

// grid coordinates for tile-based operations
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, PartialOrd, Ord)]
pub struct GridPos {
    pub x: i16,
    pub y: i16,
}

impl GridPos {
    pub fn new(x: i16, y: i16) -> Self {
        GridPos { x, y }
    }

    pub fn to_world(self, cell_size: Fixed) -> Vec2 {
        Vec2 {
            x: Fixed::from_int(self.x as i32) * cell_size,
            y: Fixed::from_int(self.y as i32) * cell_size,
        }
    }

    pub fn manhattan_distance(self, other: GridPos) -> u32 {
        ((self.x - other.x).abs() as u32) + ((self.y - other.y).abs() as u32)
    }

    // cardinal neighbours
    pub fn neighbors(self) -> [GridPos; 4] {
        [
            GridPos::new(self.x - 1, self.y),
            GridPos::new(self.x + 1, self.y),
            GridPos::new(self.x, self.y - 1),
            GridPos::new(self.x, self.y + 1),
        ]
    }

    // cardinal and diagonal neighbours
    pub fn neighbors_8(self) -> [GridPos; 8] {
        [
            GridPos::new(self.x - 1, self.y - 1),
            GridPos::new(self.x,     self.y - 1),
            GridPos::new(self.x + 1, self.y - 1),
            GridPos::new(self.x - 1, self.y),
            GridPos::new(self.x + 1, self.y),
            GridPos::new(self.x - 1, self.y + 1),
            GridPos::new(self.x,     self.y + 1),
            GridPos::new(self.x + 1, self.y + 1),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_addition_is_exact() {
        let a = Fixed::from_int(3);
        let b = Fixed::from_int(2);
        assert_eq!(a + b, Fixed::from_int(5));
    }

    #[test]
    fn fixed_multiplication_is_correct() {
        let a = Fixed::from_f64_const(1.5);
        let b = Fixed::from_f64_const(2.0);
        let result = a * b;
        assert_eq!(result, Fixed::from_int(3));
    }

    #[test]
    fn fixed_sqrt_deterministic() {
        let four = Fixed::from_int(4);
        let result = four.sqrt();
        assert_eq!(result, Fixed::from_int(2));
    }

    #[test]
    fn vec2_distance() {
        let a = Vec2::from_ints(0, 0);
        let b = Vec2::from_ints(3, 4);
        let dist = a.distance(b);
        let dist_f64 = dist.to_f64_debug();
        assert!((dist_f64 - 5.0).abs() < 0.01, "distance was {}", dist_f64);
    }

    #[test]
    fn fixed_ordering_consistent() {
        let a = Fixed::from_f64_const(1.5);
        let b = Fixed::from_f64_const(2.5);
        assert!(a < b);
        assert!(b > a);
        assert_eq!(a, Fixed::from_f64_const(1.5));
    }

    #[test]
    fn fixed_serialize_roundtrip() {
        let original = Fixed::from_f64_const(3.14159);
        let encoded = bincode::serialize(&original).unwrap();
        let decoded: Fixed = bincode::deserialize(&encoded).unwrap();
        assert_eq!(original, decoded);
    }
}
