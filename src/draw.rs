pub fn draw_quad(position: Vector, size: Vector, colour: Color) {
    canvas.draw(
        &graphics::Quad,
        DrawParam::default()
            .dest(position.into_array())
            .scale(size.into_array())
            .color(colour),
    );
}
