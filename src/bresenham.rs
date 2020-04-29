#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl From<[i32; 2]> for Point {
    fn from(x: [i32; 2]) -> Self {
        Point { x: x[0], y: x[1] }
    }
}

impl Into<[i32; 2]> for Point {
    fn into(self) -> [i32; 2] {
        [self.x, self.y]
    }
}
/// rasterize outputs an array of pixel coordinates between points p1 and p2.
/// This is the implementation of Bresenham's line algorithm using floats.
/// [https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm]
/// [https://www.youtube.com/watch?v=IDFB5CDpLDE]
pub fn rasterize(p1: Point, p2: Point) -> Vec<Point> {
    if p1 == p2 {
        return vec![p1];
    }
    let (mut dx, mut dy) = (p2.x - p1.x, p2.y - p1.y);
    let x_sign = if dx > 0 { 1 } else { -1 };
    let y_sign = if dy > 0 { 1 } else { -1 };
    dx = dx.abs();
    dy = dy.abs();
    let (mut x, mut y) = (p1.x, p1.y);
    let mut e = -0.5;
    if dy > dx {
        let delta = (dx as f32 + 1.0) / (dy as f32 + 1.0);
        (0..=dy)
            .map(|i| {
                if i == 0 {
                    return Point::new(x, y);
                }
                e += delta;
                y += y_sign;
                if e >= 0.0 {
                    x += x_sign;
                    e -= 1.0;
                }
                Point::new(x, y)
            })
            .collect()
    } else {
        let delta = (dy as f32 + 1.0) / (dx as f32 + 1.0);
        (0..=dx)
            .map(|i| {
                if i == 0 {
                    return Point::new(x, y);
                }
                e += delta;
                x += x_sign;
                if e >= 0.0 {
                    y += y_sign;
                    e -= 1.0;
                }
                Point::new(x, y)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn brezenheim_test() {
        let points = rasterize([0, 0].into(), [8, 3].into());
        let expected: Vec<Point> = vec![
            [0, 0].into(),
            [1, 0].into(),
            [2, 1].into(),
            [3, 1].into(),
            [4, 2].into(),
            [5, 2].into(),
            [6, 3].into(),
            [7, 3].into(),
            [8, 4].into(),
        ];

        for (i, &point) in points.iter().enumerate() {
            assert!(
                expected[i] == point,
                "Expected output: {:?}, what we got: {:?}",
                expected[i],
                point
            );
        }
    }
}
