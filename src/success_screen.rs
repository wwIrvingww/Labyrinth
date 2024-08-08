use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, Text, DrawParam};
use ggez::timer;
use std::time::Duration;

pub struct SuccessScreen {
    display_time: Duration,
    elapsed_time: Duration,
}

impl SuccessScreen {
    pub fn new(display_time: Duration) -> Self {
        SuccessScreen {
            display_time,
            elapsed_time: Duration::new(0, 0),
        }
    }
}

impl EventHandler for SuccessScreen {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta_time = timer::delta(ctx);
        self.elapsed_time += delta_time;

        if self.elapsed_time >= self.display_time {
            // After the display time, quit the success screen
            event::quit(ctx);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);
        let text = Text::new("SUCCESS!");
        let screen_center = ggez::mint::Point2 {
            x: graphics::drawable_size(ctx).0 / 2.0,
            y: graphics::drawable_size(ctx).1 / 2.0,
        };
        let draw_params = DrawParam::default()
            .dest(screen_center)
            .offset(ggez::mint::Point2 { x: 0.5, y: 0.5 })
            .color(Color::new(1.0, 0.0, 1.0, 1.0)); // Pink color for the text

        graphics::draw(ctx, &text, draw_params)?;
        graphics::present(ctx)?;
        Ok(())
    }
}
