use rapier2d::prelude::*;

pub fn world_to_screen_point(
    point: impl Into<Point<f32>>,
    camera_top_left: impl Into<Point<f32>>,
    camera_size: impl Into<Vector<f32>>,
    screen_top_left: impl Into<Point<f32>>,
    screen_size: impl Into<Vector<f32>>,
) -> Point<f32> {
    screen_top_left.into()
        + (point.into() - camera_top_left.into())
            .component_mul(&screen_size.into())
            .component_div(&camera_size.into())
}

pub fn world_to_screen_offset(
    offset: impl Into<Vector<f32>>,
    camera_size: impl Into<Vector<f32>>,
    screen_size: impl Into<Vector<f32>>,
) -> Vector<f32> {
    offset
        .into()
        .component_mul(&screen_size.into())
        .component_div(&camera_size.into())
}
