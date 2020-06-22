use crate::snake::Snake;
use ggez::graphics::{BlendMode, DrawParam, Drawable, Mesh, Rect};
use ggez::nalgebra as na;
use ggez::{graphics, Context, GameResult};

fn tuple_to_f32(tuple: &(usize, usize)) -> (f32, f32) {
    (tuple.0 as f32, tuple.1 as f32)
}

impl Drawable for Snake {
    fn draw(&self, ctx: &mut Context, _param: DrawParam) -> GameResult {
        // Make a grid from 100 -> 700 x, 50 -> 550 y, with dimensions of grid in confines.
        let x_interval = (600 / self.confines.0) as f32;
        let y_interval = (500 / self.confines.1) as f32;
        let head_pos = self.body.iter().next().unwrap();
        let (head_y, head_x) = tuple_to_f32(&head_pos);
        let head = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &[
                na::Point2::new(x_interval * head_x, y_interval * head_y),
                na::Point2::new(
                    x_interval * (head_x + 1.0),
                    y_interval * head_y + y_interval / 2.0,
                ),
                na::Point2::new(x_interval * head_x, y_interval * (head_y + 1.0)),
            ],
            graphics::Color::new(0.0, 1.0, 0.0, 1.0),
        )?;
        let body_rects: Vec<Mesh> = self
            .body
            .iter()
            .skip(1)
            .map(|(y, x)| {
                graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(
                        (*x as f32) * x_interval,
                        (*y as f32) * y_interval,
                        x_interval as f32,
                        y_interval as f32,
                    ),
                    graphics::Color::new(0.0, 1.0, 0.0, 1.0),
                )
                .unwrap()
            })
            .collect();
        [head]
            .iter()
            .chain(body_rects.iter())
            .map(|rect| graphics::draw(ctx, rect, (na::Point2::new(0.0, 0.0),)))
            .collect()
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
