use bevy::math::{Vec2};
use rand::random;

/// ```
/// use stars_rs::utils::Position;
/// 
/// fn main() {
///     let position = Position { x: 0.0, y: 0.0 };
/// }
/// ```
pub struct Position {
    pub x: f32,
    pub y: f32,
}

/// ```
/// use stars_rs::utils::random_in_range;
/// 
/// fn main() {
///     let x = random_in_range(0.0, 1.0);
/// }
/// ```
pub fn random_in_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random::<f32>()
}

/// ```
/// use bevy::math::{Rect};
/// use stars_rs::utils::random_in_rect_edge;
/// 
/// fn main() {
///     let x = random_in_rect_edge(-1.0, 1.0, 1.0, -1.0);
/// }
/// ```
pub fn random_in_rect_edge(left: f32, right: f32, top: f32, bottom: f32) -> Vec2 {
    if random::<bool>() {
        let x = random_in_range(left, right);
        let y = if random::<bool>() {
            top
        } else {
            bottom
        };
        Vec2::new(x, y)
    } else {
        let x = if random::<bool>() {
            right
        } else {
            left
        };
        let y = random_in_range(bottom, top);
        Vec2::new(x, y)
    }
}