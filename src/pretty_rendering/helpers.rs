use ggez::graphics::mint;

pub fn tuple_to_f32(tuple: &(i32, i32)) -> (f32, f32) {
    (tuple.0 as f32, tuple.1 as f32)
}

pub fn add_points<T: Into<mint::Point2<f32>>, X: Into<mint::Point2<f32>>>(
    first: T,
    second: X,
) -> mint::Point2<f32> {
    let first: mint::Point2<f32> = first.into();
    let second: mint::Point2<f32> = second.into();
    mint::Point2::<f32> {
        x: first.x + second.x,
        y: first.y + second.y,
    }
}
