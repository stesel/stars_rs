use bevy::math::{Size};

pub static WINDOW_SIZE: Size = Size { width: 600.0, height: 500.0 };

pub struct PositionZ {
    pub background: f32,
    pub bullet: f32,
    pub character: f32,
    pub rain: f32,
    pub menu: f32,
    pub aim: f32,
    pub fps: f32,
    pub loader: f32,
}
pub static POSITION_Z: PositionZ = PositionZ {
    background: 0.0,
    bullet: 1.0,
    character: 2.0,
    rain: 3.0,
    menu: 4.0,
    aim: 5.0,
    fps: 6.0,
    loader: 7.0,
};
