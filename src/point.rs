use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Coordinate {
    Real { x: i128, y: i128 },
    Infinity,
}

impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x, y) = match self {
            Coordinate::Real { x, y } => (x.to_string(), y.to_string()),
            Coordinate::Infinity => ("∞".to_owned(), "∞".to_owned()),
        };

        write!(f, "({x}, {y})")
    }
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub struct Point {
    pub a: i128,
    pub b: i128,
    pub coordinate: Coordinate,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point{}_{},{}", self.coordinate, self.a, self.b)
    }
}

#[allow(unused)]
impl Point {
    pub fn new(x: Option<i128>, y: Option<i128>, a: i128, b: i128) -> Result<Self, crate::Error> {
        match (x, y) {
            (None, None) => Ok(Point {
                a,
                b,
                coordinate: Coordinate::Infinity,
            }),
            (Some(x), Some(y)) => {
                if y.pow(2) != x.pow(3) + a * x + b {
                    return Err(crate::Error::ValueError(format!(
                        "({x}, {y}) is not on the curve"
                    )));
                }

                Ok(Point {
                    a,
                    b,
                    coordinate: Coordinate::Real { x, y },
                })
            }
            _ => return Err(crate::Error::ValueError("Invalid input".to_string())),
        }
    }

    pub fn add(&self, other: &Self) -> Result<Self, crate::Error> {
        if self.a != other.a || self.b != other.b {
            return Err(crate::Error::TypeError(format!(
                "Points {self}, {other} are not on the same curve"
            )));
        }

        let a = self.a;
        let b = self.b;

        match (self.coordinate, other.coordinate) {
            (
                Coordinate::Real { x, y },
                Coordinate::Real {
                    x: other_x,
                    y: other_y,
                },
            ) => {
                if x == other_x && y != other_y {
                    return Ok(Point {
                        a,
                        b,
                        coordinate: Coordinate::Infinity,
                    });
                }

                let slope = if self == other {
                    if y == 0 {
                        return Ok(Point {
                            a,
                            b,
                            coordinate: Coordinate::Infinity,
                        });
                    }

                    // s = 3x^2 + a / 2y
                    ((3.mul(x.pow(2))).add(a)).div(2.mul(y))
                } else {
                    // s = y2 - y1 / x2 - x1
                    (other_y.sub(y)).div(other_x.sub(x))
                };

                let new_x = slope.pow(2) - x - other_x;
                let new_y = slope.mul(x - new_x) - y;

                Ok(Point {
                    a,
                    b,
                    coordinate: Coordinate::Real { x: new_x, y: new_y },
                })
            }
            (Coordinate::Real { x, y }, Coordinate::Infinity) => Ok(Point {
                a,
                b,
                coordinate: Coordinate::Real { x, y },
            }),
            (Coordinate::Infinity, Coordinate::Real { x, y }) => Ok(Point {
                a,
                b,
                coordinate: Coordinate::Real { x, y },
            }),
            (Coordinate::Infinity, Coordinate::Infinity) => Ok(Point {
                a,
                b,
                coordinate: Coordinate::Infinity,
            }),
        }
    }
}
