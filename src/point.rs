pub struct Point {
    pub x: i128,
    pub y: i128,
    pub a: i128,
    pub b: i128,
}

impl Point {
    pub fn new(x: i128, y: i128, a: i128, b: i128) -> Result<Self, crate::Error> {
        if y.pow(2) != x.pow(3) + a * x + b {
            return Err(crate::Error::ValueError(format!(
                "({x}, {y}) is not on the curve",
            )));
        }

        Ok(Point { x, y, a, b })
    }
}
