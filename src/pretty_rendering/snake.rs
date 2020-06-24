use crate::snake::{Direction, Snake};
use ggez::graphics::{BlendMode, DrawParam, Drawable, Rect};
use ggez::nalgebra as na;
use ggez::{graphics, Context, GameResult};

fn tuple_to_f32(tuple: &(usize, usize)) -> (f32, f32) {
    (tuple.0 as f32, tuple.1 as f32)
}

impl Drawable for Snake {
    fn draw(&self, ctx: &mut Context, _param: DrawParam) -> GameResult {
        self.draw_head(ctx, _param)?;
        self.draw_body(ctx, _param)?;
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
    // fn draw_body(&self, ctx: &mut Context, _param: DrawParam) -> GameResult {
    //     // Make a grid from 100 -> 700 x, 50 -> 550 y, with dimensions of grid in confines.
    //     let x_interval = (600 / self.confines.0) as f32;
    //     let y_interval = (500 / self.confines.1) as f32;
    //     let body_rects: Vec<Mesh> = self
    //         .body
    //         .iter()
    //         .skip(1)
    //         .map(|(y, x)| {
    //             graphics::Mesh::new_rectangle(
    //                 ctx,
    //                 graphics::DrawMode::fill(),
    //                 graphics::Rect::new(
    //                     (*x as f32) * x_interval,
    //                     (*y as f32) * y_interval,
    //                     x_interval as f32,
    //                     y_interval as f32,
    //                 ),
    //                 graphics::Color::new(0.0, 1.0, 0.0, 1.0),
    //             )
    //             .unwrap()
    //         })
    //         .collect();
    //     body_rects
    //         .iter()
    //         .map(|rect| graphics::draw(ctx, rect, (na::Point2::new(0.0, 0.0),)))
    //         .collect()
    // }

    fn draw_head(&self, ctx: &mut Context, _param: DrawParam) -> GameResult {
        // Make a grid from 100 -> 700 x, 50 -> 550 y, with dimensions of grid in confines.
        let x_interval = (600 / self.confines.0) as f32;
        let y_interval = (500 / self.confines.1) as f32;
        let head_pos = self.body[0];
        let (head_y, head_x) = tuple_to_f32(&head_pos);
        // Line first:
        let line_start = (head_x * x_interval, head_y * y_interval + y_interval / 2.0);
        let line_end = (
            head_x * x_interval + x_interval / 4.0,
            head_y * y_interval + y_interval / 2.0,
        );
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
                na::Point2::new(
                    x_interval * head_x + x_interval / 4.0,
                    y_interval * head_y + y_interval / 4.0,
                ),
                na::Point2::new(
                    x_interval * (head_x + 1.0) - x_interval / 4.0,
                    y_interval * head_y + y_interval / 2.0,
                ),
                na::Point2::new(
                    x_interval * head_x + x_interval / 4.0,
                    y_interval * (head_y + 1.0) - y_interval / 4.0,
                ),
            ],
            graphics::Color::new(0.0, 1.0, 0.0, 1.0),
        )?;
        graphics::draw(ctx, &line, (na::Point2::new(0.0, 0.0),))?;
        graphics::draw(ctx, &arrow, (na::Point2::new(0.0, 0.0),))?;
        Ok(())
    }

    fn draw_body(&self, ctx: &mut Context, _param: DrawParam) -> GameResult {
        let x_interval = (600 / self.confines.0) as f32;
        let y_interval = (500 / self.confines.1) as f32;
        for window in Vec::from(self.body.clone()).windows(3) {
            let to = Snake::direction(window[0], window[1]);
            let from = Snake::direction(window[1], window[2]);
            let joining_mesh = Snake::get_body_line_from_directions(
                to,
                from,
                ctx,
                &graphics::Rect::new(0.0, 0.0, x_interval, y_interval),
            )?;
            let (y_to_set, x_to_set) = (
                window[1].0 as f32 * y_interval,
                window[1].1 as f32 * x_interval,
            );
            graphics::draw(
                ctx,
                &joining_mesh,
                (na::Point2::new(x_to_set as f32, y_to_set as f32),),
            )?;
        }
        Ok(())
    }

    pub fn get_body_line_from_directions(
        to: Direction,
        from: Direction,
        ctx: &mut Context,
        dimensions: &graphics::Rect,
    ) -> GameResult<graphics::Mesh> {
        let horizontal = {
            // Line first:
            let line_start = (0.0, dimensions.h / 2.0);
            let line_end = (dimensions.w, dimensions.h / 2.0);
            graphics::Mesh::new_line(
                ctx,
                &[
                    na::Point2::new(line_start.0, line_start.1),
                    na::Point2::new(line_end.0, line_end.1),
                ],
                dimensions.w / 4.0,
                graphics::Color::new(0.0, 1.0, 0.0, 1.0),
            )
            .unwrap()
        };
        Ok(match (to, from) {
            (Direction::Up, Direction::Up) => horizontal,
            (Direction::Up, Direction::Right) => horizontal,
            (Direction::Up, Direction::Down) => panic!("Not possible"),
            (Direction::Up, Direction::Left) => horizontal,
            (Direction::Right, Direction::Up) => horizontal,
            (Direction::Right, Direction::Right) => horizontal,
            (Direction::Right, Direction::Down) => horizontal,
            (Direction::Right, Direction::Left) => panic!("Not possible"),
            (Direction::Down, Direction::Up) => panic!("Not possible"),
            (Direction::Down, Direction::Right) => horizontal,
            (Direction::Down, Direction::Down) => horizontal,
            (Direction::Down, Direction::Left) => horizontal,
            (Direction::Left, Direction::Up) => horizontal,
            (Direction::Left, Direction::Right) => panic!("Not possible"),
            (Direction::Left, Direction::Down) => horizontal,
            (Direction::Left, Direction::Left) => horizontal,
        })
    }
}
