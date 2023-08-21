pub use euclid::default::{
    Box2D, Length, Point2D, Rect, Rotation2D, SideOffsets2D, Size2D, Transform2D, Translation2D,
};

pub type Vec2D<T> = euclid::default::Vector2D<T>;
pub type Angle = euclid::Angle<f32>;
pub type Scale = euclid::default::Scale<f32>;

pub use euclid;
