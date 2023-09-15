use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
// required for thumbv7em builds
#[allow(unused_imports)]
use num_traits::Float;
use num_traits::Signed;

#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr $(,)?) => {
        $crate::math::Vec2::new($x, $y)
    };
    (x: $x:expr, y: $y:expr $(,)?) => {
        $crate::math::Vec2::new($x, $y)
    };
    () => {
        $crate::math::Vec2::default()
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    /// Create a new vector.
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Set all components of the vector to the same value.
    #[inline]
    pub fn splat(v: T) -> Self
    where
        T: Clone,
    {
        Self { x: v.clone(), y: v }
    }

    /// Vector dot product.
    #[inline]
    pub fn dot(self, other: Self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        self.x * other.x + self.y * other.y
    }

    /// Vector length.
    #[inline]
    pub fn length(self) -> f32
    where
        T: Into<f32> + Clone,
    {
        let x = self.x.clone().into();
        let y = self.y.clone().into();
        (x * x + y * y).sqrt()
    }

    /// Type conversion.
    #[inline]
    pub fn cast<U>(self) -> Vec2<U>
    where
        T: Into<U>,
    {
        Vec2::new(self.x.into(), self.y.into())
    }

    /// Returns a vector with the absolute value of each component.
    #[inline]
    pub fn abs(self) -> Self
    where
        T: Signed,
    {
        Self::new(self.x.abs(), self.y.abs())
    }

    /// cross product.
    #[inline]
    pub fn cross(self, other: Self) -> T
    where
        T: Sub<Output = T> + Mul<Output = T>,
    {
        self.x * other.y - self.y * other.x
    }

    /// swap x and y
    #[inline]
    pub fn yx(self) -> Self {
        Self::new(self.y, self.x)
    }

    /// Returns the square of the vector length.
    #[inline]
    pub fn square_length(self) -> T
    where
        T: Add<Output = T> + Mul<Output = T> + Clone,
    {
        self.x.clone() * self.x + self.y.clone() * self.y
    }

    /// Returns the distance between two vectors.
    #[inline]
    pub fn distance(self, other: Self) -> f32
    where
        T: Into<f32> + Clone,
    {
        let x = self.x.clone().into() - other.x.clone().into();
        let y = self.y.clone().into() - other.y.clone().into();
        (x * x + y * y).sqrt()
    }
}

impl Vec2<f32> {
    /// Round each component to the nearest integer.
    #[inline]
    pub fn round(self) -> Self {
        Self::new(self.x.round(), self.y.round())
    }

    /// Round each component up to the nearest integer.
    #[inline]
    pub fn ceil(self) -> Self {
        Self::new(self.x.ceil(), self.y.ceil())
    }

    /// Round each component down to the nearest integer.
    #[inline]
    pub fn floor(self) -> Self {
        Self::new(self.x.floor(), self.y.floor())
    }

    /// Normalize the vector.
    #[inline]
    pub fn normalize(self) -> Self {
        let length = self.length();
        if length == 0.0 {
            return Self::ZERO;
        }
        self / length
    }

    /// scale the vector to the given length.
    #[inline]
    pub fn with_length(self, length: f32) -> Self {
        self.normalize() * length
    }

    /// Rotate the vector around a center point by the given angle in radians.
    #[inline]
    pub fn rotate_around(self, center: Self, radians: f32) -> Self {
        let (sin, cos) = radians.sin_cos();
        let x = self.x - center.x;
        let y = self.y - center.y;
        Self::new(x * cos - y * sin + center.x, x * sin + y * cos + center.y)
    }

    /// Rotate the vector by the given angle in radians.
    #[inline]
    pub fn rotate(self, radians: f32) -> Self {
        let (sin, cos) = radians.sin_cos();
        Self::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
    }

    /// Scale the vector by another vector.
    #[inline]
    pub fn scale(self, other: Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y)
    }

    /// Translate the vector by another vector.
    #[inline]
    pub fn translate(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    /// Project the vector onto another vector.
    #[inline]
    pub fn project_onto(self, other: Self) -> Self {
        let other = other.normalize();
        other * self.dot(other)
    }

    /// Reflect the vector around a normal.
    #[inline]
    pub fn reflect(self, normal: Self) -> Self {
        self - normal * 2.0 * self.dot(normal)
    }
}

impl From<Vec2<i32>> for Vec2<f32> {
    #[inline]
    fn from(v: Vec2<i32>) -> Self {
        Self {
            x: v.x as _,
            y: v.y as _,
        }
    }
}

impl From<Vec2<f32>> for Vec2<i32> {
    #[inline]
    fn from(v: Vec2<f32>) -> Self {
        Self {
            x: v.x as _,
            y: v.y as _,
        }
    }
}

impl<T> From<(T, T)> for Vec2<T> {
    #[inline]
    fn from(v: (T, T)) -> Self {
        Self { x: v.0, y: v.1 }
    }
}

impl<T> From<Vec2<T>> for (T, T) {
    #[inline]
    fn from(val: Vec2<T>) -> Self {
        (val.x, val.y)
    }
}

impl<T> From<Vec2<T>> for [T; 2] {
    #[inline]
    fn from(val: Vec2<T>) -> Self {
        [val.x, val.y]
    }
}

impl<T: Add<Output = T> + Clone> Add<T> for Vec2<T> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        Self::new(self.x + rhs.clone(), self.y + rhs)
    }
}

impl<T: Add<Output = T>> Add<Self> for Vec2<T> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: AddAssign<T> + Clone> AddAssign<T> for Vec2<T> {
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs.clone();
        self.y += rhs;
    }
}

impl<T: AddAssign<T>> AddAssign<Self> for Vec2<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub<Output = T> + Clone> Sub<T> for Vec2<T> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        Self::new(self.x - rhs.clone(), self.y - rhs)
    }
}

impl<T: Sub<Output = T>> Sub<Self> for Vec2<T> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: SubAssign<T> + Clone> SubAssign<T> for Vec2<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs.clone();
        self.y -= rhs;
    }
}

impl<T: SubAssign<T>> SubAssign<Self> for Vec2<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Mul<Output = T> + Clone> Mul<T> for Vec2<T> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs.clone(), self.y * rhs)
    }
}

impl<T: MulAssign<T>> MulAssign<Self> for Vec2<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: Div<Output = T> + Clone> Div<T> for Vec2<T> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs.clone(), self.y / rhs)
    }
}

impl<T: DivAssign<T>> DivAssign<Self> for Vec2<T> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Vec2<i8> {
    pub const ZERO: Self = Self { x: 0, y: 0 };
}

impl Vec2<u8> {
    pub const ZERO: Self = Self { x: 0, y: 0 };
}

impl Vec2<i32> {
    pub const ZERO: Self = Self { x: 0, y: 0 };
}

impl Vec2<u32> {
    pub const ZERO: Self = Self { x: 0, y: 0 };
}

impl Vec2<isize> {
    pub const ZERO: Self = Self { x: 0, y: 0 };
}

impl Vec2<usize> {
    pub const ZERO: Self = Self { x: 0, y: 0 };
}

impl Vec2<f32> {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
}

#[macro_export]
macro_rules! size {
    ($w:expr, $h:expr $(,)?) => {
        $crate::math::Size::new($w, $h)
    };
    (w: $w:expr, h: $h:expr $(,)?) => {
        $crate::math::Size::new($w, $h)
    };
    (width: $w:expr, height: $h:expr $(,)?) => {
        $crate::math::Size::new($w, $h)
    };
    (square: $square: expr $(,)?) => {
        $crate::math::Size::splat($square)
    };
    () => {
        $crate::math::Size::default()
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

impl<T> Size<T> {
    /// Create a new size.
    #[inline]
    pub const fn new(width: T, height: T) -> Self {
        Self { width, height }
    }

    /// Set all components of the size to the same value.
    #[inline]
    pub fn splat(v: T) -> Self
    where
        T: Clone,
    {
        Self {
            width: v.clone(),
            height: v,
        }
    }

    /// Type conversion.
    #[inline]
    pub fn cast<U>(self) -> Size<U>
    where
        T: Into<U>,
    {
        Size::new(self.width.into(), self.height.into())
    }

    /// Returns the area of the size.
    #[inline]
    pub fn area(self) -> T
    where
        T: Mul<Output = T>,
    {
        self.width * self.height
    }

    /// Returns the aspect ratio of the size.
    #[inline]
    pub fn aspect_ratio(self) -> f32
    where
        T: Into<f32> + Clone,
    {
        let width = self.width.clone().into();
        let height = self.height.clone().into();
        width / height
    }
}

impl Size<u8> {
    pub const ZERO: Self = Self::new(0, 0);
    pub const ONE: Self = Self::new(1, 1);
    pub const TILE16: Self = Self::new(16, 16);
    pub const TILE32: Self = Self::new(32, 32);
    pub const TILE64: Self = Self::new(64, 64);
}

impl Size<i8> {
    pub const ZERO: Self = Self::new(0, 0);
    pub const ONE: Self = Self::new(1, 1);
    pub const TILE16: Self = Self::new(16, 16);
    pub const TILE32: Self = Self::new(32, 32);
    pub const TILE64: Self = Self::new(64, 64);
}

impl Size<u16> {
    pub const ZERO: Self = Self::new(0, 0);
    pub const ONE: Self = Self::new(1, 1);
    pub const TILE16: Self = Self::new(16, 16);
    pub const TILE32: Self = Self::new(32, 32);
    pub const TILE64: Self = Self::new(64, 64);
}

impl Size<i16> {
    pub const ZERO: Self = Self::new(0, 0);
    pub const ONE: Self = Self::new(1, 1);
    pub const TILE16: Self = Self::new(16, 16);
    pub const TILE32: Self = Self::new(32, 32);
    pub const TILE64: Self = Self::new(64, 64);
}

impl Size<u32> {
    pub const ZERO: Self = Self::new(0, 0);
    pub const ONE: Self = Self::new(1, 1);
    pub const TILE16: Self = Self::new(16, 16);
    pub const TILE32: Self = Self::new(32, 32);
    pub const TILE64: Self = Self::new(64, 64);
}

impl Size<i32> {
    pub const ZERO: Self = Self::new(0, 0);
    pub const ONE: Self = Self::new(1, 1);
    pub const TILE16: Self = Self::new(16, 16);
    pub const TILE32: Self = Self::new(32, 32);
    pub const TILE64: Self = Self::new(64, 64);
}

impl Size<usize> {
    pub const ZERO: Self = Self::new(0, 0);
    pub const ONE: Self = Self::new(1, 1);
    pub const TILE16: Self = Self::new(16, 16);
    pub const TILE32: Self = Self::new(32, 32);
    pub const TILE64: Self = Self::new(64, 64);
}

impl Size<isize> {
    pub const ZERO: Self = Self::new(0, 0);
    pub const ONE: Self = Self::new(1, 1);
    pub const TILE16: Self = Self::new(16, 16);
    pub const TILE32: Self = Self::new(32, 32);
    pub const TILE64: Self = Self::new(64, 64);
}

impl Size<f32> {
    pub const ZERO: Self = Self::new(0.0, 0.0);
    pub const ONE: Self = Self::new(1.0, 1.0);
    pub const TILE16: Self = Self::new(16.0, 16.0);
    pub const TILE32: Self = Self::new(32.0, 32.0);
    pub const TILE64: Self = Self::new(64.0, 64.0);
}

#[macro_export]
macro_rules! rect {
    (x: $x:expr, y: $y:expr, w: $w:expr, h: $h:expr $(,)?) => {
        $crate::math::Rect::new($x, $y, $w, $h)
    };
    (x: $x:expr, y: $y:expr, width: $w:expr, height: $h:expr $(,)?) => {
        $crate::math::Rect::new($x, $y, $w, $h)
    };
    (pos: $p:expr, size: $s:expr $(,)?) => {
        $crate::math::Rect::from_point_and_size($p, $s)
    };
    () => {
        $crate::math::Rect::default()
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T> Rect<T> {
    /// Create a new rectangle.
    #[inline]
    pub const fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Create a new rectangle from a position and a size.
    #[inline]
    pub fn from_pos_and_size(pos: Vec2<T>, size: Size<T>) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            width: size.width,
            height: size.height,
        }
    }

    /// Get the top-left corner of the rectangle.
    #[inline]
    pub fn pos(&self) -> Vec2<T>
    where
        T: Clone,
    {
        Vec2::new(self.x.clone(), self.y.clone())
    }

    /// Get the size of the rectangle.
    #[inline]
    pub fn size(&self) -> Size<T>
    where
        T: Clone,
    {
        Size::new(self.width.clone(), self.height.clone())
    }

    /// Type conversion.
    #[inline]
    pub fn cast<U>(self) -> Rect<U>
    where
        T: Into<U>,
    {
        Rect::new(
            self.x.into(),
            self.y.into(),
            self.width.into(),
            self.height.into(),
        )
    }

    /// Returns the area of the rectangle.
    #[inline]
    pub fn area(&self) -> T
    where
        T: Mul<Output = T> + Clone,
    {
        self.width.clone() * self.height.clone()
    }

    /// Returns the aspect ratio of the rectangle.
    #[inline]
    pub fn aspect_ratio(&self) -> f32
    where
        T: Into<f32> + Clone,
    {
        let width = self.width.clone().into();
        let height = self.height.clone().into();
        width / height
    }

    /// Get the intersection of two rectangles.
    #[inline]
    pub fn intersection(&self, other: &Self) -> Option<Self>
    where
        T: PartialOrd + Add<T, Output = T> + Sub<T, Output = T> + Default + Clone + Copy,
    {
        let min = |a: T, b: T| if a < b { a } else { b };
        let max = |a: T, b: T| if a > b { a } else { b };
        let x = max(self.x, other.x);
        let y = max(self.y, other.y);
        let width = min(self.x + self.width, other.x + other.width) - x;
        let height = min(self.y + self.height, other.y + other.height) - y;
        if width <= T::default() || height <= T::default() {
            None
        } else {
            Some(Self::new(x, y, width, height))
        }
    }
}

impl Rect<f32> {
    /// Scale the rectangle size by another vector.
    #[inline]
    pub fn scale(self, scale: Vec2<f32>) -> Self {
        Self::new(self.x, self.y, self.width * scale.x, self.height * scale.y)
    }

    /// Translate the rectangle by another vector.
    #[inline]
    pub fn translate(self, delta: Vec2<f32>) -> Self {
        Self::new(self.x + delta.x, self.y + delta.y, self.width, self.height)
    }

    /// Returns the center of the rectangle.
    #[inline]
    pub fn center(&self) -> Vec2<f32> {
        Vec2::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    /// Check if the rectangle intersects another rectangle.
    #[inline]
    pub fn intersects(&self, other: &Self) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    /// Check if the rectangle contains a point.
    #[inline]
    pub fn contains_point(&self, point: Vec2<f32>) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }

    /// Check if the rectangle contains another rectangle.
    #[inline]
    pub fn contains_rect(&self, other: &Self) -> bool {
        self.x <= other.x
            && self.x + self.width >= other.x + other.width
            && self.y <= other.y
            && self.y + self.height >= other.y + other.height
    }
}

impl From<Rect<f32>> for sys::PDRect {
    #[inline]
    fn from(val: Rect<f32>) -> Self {
        sys::PDRect {
            x: val.x,
            y: val.y,
            width: val.width,
            height: val.height,
        }
    }
}

impl From<sys::PDRect> for Rect<f32> {
    #[inline]
    fn from(rect: sys::PDRect) -> Self {
        Self {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        }
    }
}

impl From<sys::CollisionPoint> for Vec2<f32> {
    #[inline]
    fn from(v: sys::CollisionPoint) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<sys::CollisionVector> for Vec2<i32> {
    #[inline]
    fn from(v: sys::CollisionVector) -> Self {
        Self { x: v.x, y: v.y }
    }
}

pub struct SideOffsets<T> {
    pub left: T,
    pub right: T,
    pub top: T,
    pub bottom: T,
}

impl<T> SideOffsets<T> {
    #[inline]
    pub fn new(left: T, right: T, top: T, bottom: T) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    #[inline]
    pub fn splat(v: T) -> Self
    where
        T: Clone,
    {
        Self {
            left: v.clone(),
            right: v.clone(),
            top: v.clone(),
            bottom: v,
        }
    }
}

impl SideOffsets<i32> {
    pub const ZERO: Self = Self {
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
    };
}

impl From<SideOffsets<i32>> for sys::LCDRect {
    #[inline]
    fn from(val: SideOffsets<i32>) -> Self {
        sys::LCDRect {
            left: val.left,
            right: val.right,
            top: val.top,
            bottom: val.bottom,
        }
    }
}
