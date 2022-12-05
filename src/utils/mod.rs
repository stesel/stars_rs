use bevy::math::Vec2;
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
/// use stars_rs::utils::Size;
///
/// fn main() {
///     let size = Size { width: 1.0, height: 2.0 };
/// }
/// ```
pub struct Size {
    pub width: f32,
    pub height: f32,
}

/// ```
/// use stars_rs::utils::BoundingRect;
///
/// fn main() {
///     let rect = BoundingRect { x: 0.0, y: 0.0, width: 1.0, height: 1.0 };
/// }
/// ```
pub struct BoundingRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// ```
/// use stars_rs::utils::{BoundingRect, GetBoundingRect};
///
/// fn main() {
///     struct Item();
///
///     impl GetBoundingRect for Item {
///         fn get_bounding_rect(&self) -> BoundingRect {
///             BoundingRect { x: 0.0, y: 0.0, width: 1.0, height: 1.0 }
///         }
///     }
/// }
/// ```
///
pub trait GetBoundingRect {
    fn get_bounding_rect(&self) -> BoundingRect;
}

/// ```
/// use stars_rs::utils::{SetSpeed};
/// use bevy::math::{Vec2};
///
/// fn main() {
///     struct Item {
///         speed: Vec2,
///     };
///
///     impl SetSpeed for Item {
///         fn set_speed(&mut self, speed: Vec2) {
///             self.speed = speed;
///         }
///     }
/// }
/// ```
pub trait SetSpeed {
    fn set_speed(&mut self, speed: Vec2);
}

/// ```
/// use stars_rs::utils::{IsActive};
///
/// fn main() {
///     struct Item {
///         active: bool,
///     };
///
///     impl IsActive for Item {
///         fn get_active(&self) -> bool {
///             self.active
///         }
///
///         fn set_active(&mut self, active: bool) {
///             self.active = active;
///         }
///     }
/// }
/// ```
pub trait IsActive {
    fn get_active(&self) -> bool;

    fn set_active(&mut self, active: bool);
}

/// ```
/// use stars_rs::utils::{BoundingRect, hit_test};
///
/// fn main() {
///     let lhr = BoundingRect { x: 0.0, y: 0.0, width: 1.0, height: 1.0 };
///     let rhr = BoundingRect { x: 1.0, y: 1.0, width: 1.0, height: 1.0 };
///
///     let hit = hit_test(lhr, rhr);
/// }
/// ```
pub fn hit_test(lhr: BoundingRect, rhr: BoundingRect) -> bool {
    (lhr.x - rhr.x).abs() < (lhr.width + rhr.width) / 4.0
        && (lhr.y - rhr.y).abs() < (lhr.height + rhr.height) / 4.0
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
/// use stars_rs::utils::random_in_rect_edge;
///
/// fn main() {
///     let x = random_in_rect_edge(-1.0, 1.0, 1.0, -1.0);
/// }
/// ```
pub fn random_in_rect_edge(left: f32, right: f32, top: f32, bottom: f32) -> Vec2 {
    if random::<bool>() {
        let x = random_in_range(left, right);
        let y = if random::<bool>() { top } else { bottom };
        Vec2::new(x, y)
    } else {
        let x = if random::<bool>() { right } else { left };
        let y = random_in_range(bottom, top);
        Vec2::new(x, y)
    }
}
