use crate::pretty_rendering::helpers::add_points;

use ggez::graphics::{mint, BlendMode, DrawMode, DrawParam, Drawable, Rect};
use ggez::nalgebra as na;
use ggez::{graphics, Context, GameResult};
use std::collections::HashSet;

pub struct Apples<'a> {
    inner: &'a HashSet<crate::Apple>,
    container: Rect,
    confines: (i32, i32),
}

impl<'a> Apples<'a> {
    pub fn new(inner: &'a HashSet<crate::Apple>, container: Rect, confines: (i32, i32)) -> Self {
        Apples {
            inner,
            container,
            confines,
        }
    }
}

impl<'a> Drawable for Apples<'a> {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        let x_interval = self.container.w / self.confines.0 as f32;
        let y_interval = self.container.h / self.confines.1 as f32;
        let draw_offsets: Vec<_> = self
            .inner
            .iter()
            .map(|apple| -> mint::Point2<f32> {
                let x = apple.location.1 as f32 * x_interval;
                let y = apple.location.0 as f32 * y_interval;
                add_points(na::Point2::new(x, y), param.dest)
            })
            .collect();
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            graphics::Rect::new(
                x_interval / 3.0,
                y_interval / 3.0,
                x_interval / 3.0,
                y_interval / 3.0,
            ),
            graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        )
        .unwrap();
        draw_offsets
            .into_iter()
            .map(|offset: mint::Point2<f32>| graphics::draw(ctx, &rect, (offset,)))
            .collect()
    }

    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        Some(self.container)
    }

    fn set_blend_mode(&mut self, _mode: Option<BlendMode>) {}

    fn blend_mode(&self) -> Option<BlendMode> {
        None
    }
}
