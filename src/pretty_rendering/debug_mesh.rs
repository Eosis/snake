use ggez::graphics::{draw, BlendMode, Color, DrawParam, Drawable, Mesh, Rect};
use ggez::nalgebra as na;
use ggez::Context;
use ggez::GameResult;

pub struct DebugMesh {
    pub rows: usize,
    pub columns: usize,
    pub container: ggez::graphics::Rect,
}

impl Drawable for DebugMesh {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        let (x_interval, y_interval) = (
            self.container.w / self.columns as f32,
            self.container.h / self.rows as f32,
        );
        for x in 0..self.columns + 1 {
            let line = Mesh::new_line(
                ctx,
                &[
                    na::Point2::new(x as f32 * x_interval, 0.0),
                    na::Point2::new(x as f32 * x_interval, self.container.w),
                ],
                1.0,
                Color::new(0.0, 0.0, 0.5, 1.0),
            )?;
            draw(ctx, &line, param)?;
        }
        for y in 0..self.rows + 1 {
            let line = Mesh::new_line(
                ctx,
                &[
                    na::Point2::new(0.0, y as f32 * y_interval),
                    na::Point2::new(self.container.h, y as f32 * y_interval),
                ],
                1.0,
                Color::new(0.0, 0.0, 0.5, 1.0),
            )?;
            draw(ctx, &line, param)?;
        }
        Ok(())
    }

    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        Some(self.container)
    }

    fn set_blend_mode(&mut self, _mode: Option<BlendMode>) {}

    fn blend_mode(&self) -> Option<BlendMode> {
        None
    }
}
