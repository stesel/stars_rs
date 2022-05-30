use rand::random;

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