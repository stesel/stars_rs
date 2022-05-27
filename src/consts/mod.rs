use bevy::math::{Size};

pub static WINDOW_SIZE: Size = Size { width: 600.0, height: 500.0 };


pub struct PositionZ {
    pub background: f32,
    pub character: f32,
    pub aim: f32,
    pub fps: f32,
}
pub static ELEMENT_POSITION_Z: PositionZ = PositionZ {
    background: 0.0,
    character: 1.0,
    aim: 2.0,
    fps: 3.0,
};
