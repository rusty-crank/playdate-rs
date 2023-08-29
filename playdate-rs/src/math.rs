use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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
    pub const ONE: Self = Self { x: 1, y: 1 };
}

impl Vec2<u8> {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const ONE: Self = Self { x: 1, y: 1 };
}

impl Vec2<i32> {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const ONE: Self = Self { x: 1, y: 1 };
}

impl Vec2<u32> {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const ONE: Self = Self { x: 1, y: 1 };
}

impl Vec2<isize> {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const ONE: Self = Self { x: 1, y: 1 };
}

impl Vec2<usize> {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const ONE: Self = Self { x: 1, y: 1 };
}

impl Vec2<f32> {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };
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
}

impl Into<sys::PDRect> for Rect<f32> {
    #[inline]
    fn into(self) -> sys::PDRect {
        sys::PDRect {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
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

impl Into<sys::LCDRect> for SideOffsets<i32> {
    #[inline]
    fn into(self) -> sys::LCDRect {
        sys::LCDRect {
            left: self.left,
            right: self.right,
            top: self.top,
            bottom: self.bottom,
        }
    }
}
