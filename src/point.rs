use crate::{field_element::FieldElement, Pow};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Coordinate<T> {
    Real(T, T),
    Infinity,
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub struct Point<T> {
    pub a: T,
    pub b: T,
    pub coordinate: Coordinate<T>,
}

impl<T: std::fmt::Display> std::fmt::Display for Coordinate<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x, y) = match self {
            Coordinate::Real(x, y) => (x.to_string(), y.to_string()),
            Coordinate::Infinity => ("∞".to_owned(), "∞".to_owned()),
        };

        write!(f, "({x}, {y})")
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point{}_{},{}", self.coordinate, self.a, self.b)
    }
}

#[allow(unused)]
impl<T> Point<T>
where
    T: std::fmt::Display
        + crate::Pow<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Div<Output = T>
        + Copy
        + PartialEq,
{
    pub fn new(x: Option<T>, y: Option<T>, a: T, b: T) -> Result<Self, crate::Error> {
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
                    coordinate: Coordinate::Real(x, y),
                })
            }
            _ => return Err(crate::Error::ValueError("Invalid input".to_string())),
        }
    }
}

pub trait Add {
    type Output;

    fn add(&self, other: &Self) -> Result<Self::Output, crate::Error>;
}

impl crate::Pow for i128 {
    type Output = i128;

    fn pow(self, rhs: i32) -> Self::Output {
        self.pow(rhs as u32)
    }
}

impl Add for Point<i128> {
    type Output = Point<i128>;

    fn add(&self, other: &Self) -> Result<Self, crate::Error> {
        if self.a != other.a || self.b != other.b {
            return Err(crate::Error::TypeError(format!(
                "Points {self}, {other} are not on the same curve"
            )));
        }

        let a = self.a;
        let b = self.b;

        match (self.coordinate, other.coordinate) {
            (Coordinate::Real(x, y), Coordinate::Real(other_x, other_y)) => {
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
                    ((3 * (x.pow(2))) + a) / (2 * y)
                } else {
                    // s = y2 - y1 / x2 - x1
                    (other_y - y) / (other_x - x)
                };

                let new_x = slope.pow(2) - x - other_x;
                let new_y = slope * (x - new_x) - y;

                Ok(Point {
                    a,
                    b,
                    coordinate: Coordinate::Real(new_x, new_y),
                })
            }
            (Coordinate::Real(x, y), Coordinate::Infinity) => Ok(Point {
                a,
                b,
                coordinate: Coordinate::Real(x, y),
            }),
            (Coordinate::Infinity, Coordinate::Real(x, y)) => Ok(Point {
                a,
                b,
                coordinate: Coordinate::Real(x, y),
            }),
            (Coordinate::Infinity, Coordinate::Infinity) => Ok(Point {
                a,
                b,
                coordinate: Coordinate::Infinity,
            }),
        }
    }
}

impl<const P: u128> Add for Point<crate::field_element::FieldElement<P>> {
    type Output = Point<crate::field_element::FieldElement<P>>;

    fn add(&self, other: &Self) -> Result<Self::Output, crate::Error> {
        if self.a != other.a || self.b != other.b {
            return Err(crate::Error::TypeError(format!(
                "Points {self}, {other} are not on the same curve"
            )));
        }

        let a = self.a;
        let b = self.b;

        match (self.coordinate, other.coordinate) {
            (Coordinate::Real(x, y), Coordinate::Real(other_x, other_y)) => {
                if x == other_x && y != other_y {
                    return Ok(Point {
                        a,
                        b,
                        coordinate: Coordinate::Infinity,
                    });
                }

                let slope = if self == other {
                    let zero = FieldElement::<P>::new(0)?;

                    if y == zero {
                        return Ok(Point {
                            a,
                            b,
                            coordinate: Coordinate::Infinity,
                        });
                    }

                    let three = FieldElement::<P>::new(3)?;
                    let two = FieldElement::<P>::new(2)?;

                    // s = 3x^2 + a / 2y
                    ((three * (x.pow(2))) + a) / (two * y)
                } else {
                    // s = y2 - y1 / x2 - x1
                    (other_y - y) / (other_x - x)
                };

                let new_x = slope.pow(2) - x - other_x;
                let new_y = slope * (x - new_x) - y;

                Ok(Point {
                    a,
                    b,
                    coordinate: Coordinate::Real(new_x, new_y),
                })
            }
            (Coordinate::Real(x, y), Coordinate::Infinity) => Ok(Point {
                a,
                b,
                coordinate: Coordinate::Real(x, y),
            }),
            (Coordinate::Infinity, Coordinate::Real(x, y)) => Ok(Point {
                a,
                b,
                coordinate: Coordinate::Real(x, y),
            }),
            (Coordinate::Infinity, Coordinate::Infinity) => Ok(Point {
                a,
                b,
                coordinate: Coordinate::Infinity,
            }),
        }
    }
}
