use crate::pretty_rendering::helpers::*;
use crate::snake::{Direction, Snake};
use ggez::graphics::{mint, BlendMode, DrawParam, Drawable, Rect};
use ggez::nalgebra as na;
use ggez::{graphics, Context, GameResult};
use std::f32::consts::PI;

impl Drawable for Snake {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        self.draw_head(ctx, param)?;
        self.draw_body(ctx, param)?;
        Ok(())
    }

    /// Returns a bounding box in the form of a `Rect`.
    ///
    /// It returns `Option` because some `Drawable`s may have no bounding box
    /// (an empty `SpriteBatch` for example).
    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        Some(graphics::Rect::new(100.0, 700.0, 600.0, 500.0))
    }

    /// Sets the blend mode to be used when drawing this drawable.
    /// This overrides the general [`graphics::set_blend_mode()`](fn.set_blend_mode.html).
    /// If `None` is set, defers to the blend mode set by
    /// `graphics::set_blend_mode()`.
    fn set_blend_mode(&mut self, _mode: Option<BlendMode>) {}

    /// Gets the blend mode to be used when drawing this drawable.
    fn blend_mode(&self) -> Option<BlendMode> {
        None
    }
}

impl Snake {
    fn draw_head(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        let x_interval = self.confines_size.0 / self.confines.0 as f32;
        let y_interval = self.confines_size.1 / self.confines.1 as f32;
        let head_pos = self.body[0];
        let (head_y, head_x) = tuple_to_f32(&head_pos);
        // Line first:
        let line_start = (0.0, y_interval / 2.0);
        let line_end = (x_interval / 4.0, y_interval / 2.0);
        let line = graphics::Mesh::new_line(
            ctx,
            &[
                na::Point2::new(line_start.0, line_start.1),
                na::Point2::new(line_end.0, line_end.1),
            ],
            x_interval / 4.0,
            graphics::Color::new(0.0, 1.0, 0.0, 1.0),
        )?;

        let arrow = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &[
                na::Point2::new(x_interval / 4.0, y_interval / 4.0),
                na::Point2::new(x_interval * 3.0 / 4.0, y_interval / 2.0),
                na::Point2::new(x_interval / 4.0, y_interval * 3.0 / 4.0),
            ],
            graphics::Color::new(0.0, 1.0, 0.0, 1.0),
        )?;
        let head_rotation = match Snake::head_direction(self.body.iter()) {
            Direction::Up => 3.0 * PI / 2.0,
            Direction::Right => 0.0,
            Direction::Down => PI / 2.0,
            Direction::Left => PI,
        };
        let new_dest: mint::Point2<f32> = add_points(
            na::Point2::new(x_interval * head_x, y_interval * head_y),
            param.dest,
        );
        let rot = DrawParam {
            rotation: head_rotation,
            dest: new_dest,
            offset: na::Point2::new(x_interval / 2.0, y_interval / 2.0).into(),
            ..Default::default()
        };
        graphics::draw(ctx, &line, rot)?;
        graphics::draw(ctx, &arrow, rot)?;
        Ok(())
    }

    fn body_pos_to_na_point(
        (y, x): (i32, i32),
        x_interval: f32,
        y_interval: f32,
    ) -> na::Point2<f32> {
        na::Point2::new(
            x as f32 * x_interval + x_interval / 2.0,
            y as f32 * y_interval + y_interval / 2.0,
        )
    }

    fn draw_body(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        let x_interval = self.confines_size.0 / self.confines.0 as f32;
        let y_interval = self.confines_size.0 / self.confines.1 as f32;
        let body_points: Vec<_> = self
            .body
            .iter()
            .map(|x| Snake::body_pos_to_na_point(*x, x_interval, y_interval))
            .collect();
        let body = graphics::Mesh::new_line(
            ctx,
            &body_points,
            x_interval / 4.0,
            graphics::Color::new(0.0, 1.0, 0.0, 1.0),
        )?;
        graphics::draw(ctx, &body, (param.dest,))
    }
}
