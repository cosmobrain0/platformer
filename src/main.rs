use ggez::conf::WindowMode;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam};
use ggez::{Context, ContextBuilder, GameResult};

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("2D Platformer", "CosmoBrain")
        .window_mode(WindowMode::default().dimensions(1920.0 / 2.0, 1080.0 / 2.0))
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = Platformer::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}

struct Platformer {
    // Your state here...
}

impl Platformer {
    pub fn new(_ctx: &mut Context) -> Platformer {
        // Load/create resources such as images here.
        Platformer {
            // ...
        }
    }
}

impl EventHandler for Platformer {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 120;
        while ctx.time.check_update_time(DESIRED_FPS) {
            // update one frame
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        canvas.set_screen_coordinates(graphics::Rect {
            x: 0.0,
            y: 0.0,
            w: 1920.0,
            h: 1080.0,
        });

        canvas.draw(
            &graphics::Quad,
            DrawParam::default()
                .dest([20.0, 20.0])
                .scale([1880.0, 50.0])
                .color(Color::BLACK),
        );

        canvas.finish(ctx)
    }
}
