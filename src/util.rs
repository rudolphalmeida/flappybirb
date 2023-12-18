use nalgebra as na;

pub fn vertically_centered_position(
    container_size: na::Vector2<f32>,
    element_size: na::Vector2<f32>,
) -> f32 {
    container_size.x / 2.0 - (element_size.x / 2.0)
}

pub fn horizontally_centered_position(
    container_size: na::Vector2<f32>,
    element_size: na::Vector2<f32>,
) -> f32 {
    container_size.y / 2.0 - (element_size.y / 2.0)
}
