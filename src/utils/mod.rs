use rand::random;

pub fn random_in_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random::<f32>()
}