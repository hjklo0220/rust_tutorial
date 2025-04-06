
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl Point<f32> {
    pub fn x(&self) -> &f32 {
        &self.x
    }

    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub struct PointV2<X1, Y1> {
    pub x: X1,
    pub y: Y1,
}

impl<X1, Y1> PointV2<X1, Y1> {
    pub fn mixup<X2, Y2>(self, other: PointV2<X2, Y2>) -> PointV2<X1, Y2> {
        PointV2 {
            x: self.x,
            y: other.y,
        }
    }
}