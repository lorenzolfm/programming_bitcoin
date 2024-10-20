use std::ops::Mul;

use crate::Error;

#[derive(Copy, Clone, Debug)]
pub enum Coordinate<T> {
    Real { x: T, y: T },
    Infinity,
}

#[derive(Copy, Clone, Debug)]
pub struct Point<T> {
    pub a: T,
    pub b: T,
    pub coord: Coordinate<T>,
}

#[allow(unused)]
impl Point<i64> {
    fn new(x: Option<i64>, y: Option<i64>, a: i64, b: i64) -> Result<Self, Error> {
        match (x, y) {
            (Some(x_val), Some(y_val)) => {
                if y_val.pow(2) != x_val.pow(3) + std::ops::Mul::mul(a, x_val) + b {
                    return Err(Error::ValueError);
                }

                Ok(Self {
                    a,
                    b,
                    coord: Coordinate::Real { x: x_val, y: y_val },
                })
            }
            (None, None) => Ok(Self {
                a,
                b,
                coord: Coordinate::Infinity,
            }),
            _ => return Err(Error::ValueError),
        }
    }

    fn add(&self, other: &Self) -> Result<Self, Error> {
        if self.a != other.a || self.b != other.b {
            return Err(Error::ValueError);
        }

        let a = self.a;
        let b = self.b;

        match (self.coord, other.coord) {
            (
                Coordinate::Real { x, y },
                Coordinate::Real {
                    x: other_x,
                    y: other_y,
                },
            ) => {
                // Additive inverse: same x, different y. Returns the point at infinity
                if x == other_x && y != other_y {
                    return Ok(Point {
                        a: self.a,
                        b: self.b,
                        coord: Coordinate::Infinity,
                    });
                }

                let slope = if self == other {
                    if y == 0 {
                        return Ok(Point {
                            a,
                            b,
                            coord: Coordinate::Infinity,
                        });
                    }

                    (3.mul(x.pow(2)) + a) / 2.mul(y)
                } else {
                    (other_y - y) / (other_x - x)
                };

                let new_x = slope.pow(2) - x - other_x;
                let new_y = slope.mul(x - new_x) - y;

                Ok(Point {
                    a,
                    b,
                    coord: Coordinate::Real { x: new_x, y: new_y },
                })
            }
            (Coordinate::Real { x, y }, Coordinate::Infinity) => Ok(*other),
            (Coordinate::Infinity, Coordinate::Real { x, y }) => Ok(*self),
            (Coordinate::Infinity, Coordinate::Infinity) => Ok(Point {
                a: self.a,
                b: self.b,
                coord: Coordinate::Infinity,
            }),
        }
    }
}

impl<T: PartialEq> std::cmp::PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.a != other.a || self.b != other.b {
            return false;
        }

        match (&self.coord, &other.coord) {
            (
                Coordinate::Real { x, y },
                Coordinate::Real {
                    x: other_x,
                    y: other_y,
                },
            ) => x == other_x && y == other_y,
            (Coordinate::Infinity, Coordinate::Infinity) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;

    #[test]
    fn point() {
        let a = 5;
        let b = 7;

        let p1 = Point::new(Some(2), Some(4), a, b);
        assert!(p1.is_err());

        let p2 = Point::new(Some(-1), Some(-1), a, b);
        assert!(p2.is_ok());

        let p3 = Point::new(Some(18), Some(77), a, b);
        assert!(p3.is_ok());

        let p4 = Point::new(Some(5), Some(7), a, b);
        assert!(p4.is_err());
    }

    #[test]
    fn add() {
        let a = 5;
        let b = 7;

        let p1 = Point::new(Some(2), Some(5), a, b).unwrap();
        let p2 = Point::new(Some(-1), Some(-1), a, b).unwrap();
        let p3 = p1.add(&p2).unwrap();

        let expected = Point::new(Some(3), Some(-7), a, b).unwrap();

        assert_eq!(p3, expected);

        let p1 = Point::new(Some(-1), Some(-1), a, b).unwrap();
        let p3 = p1.add(&p1).unwrap();

        let expected = Point::new(Some(18), Some(77), a, b).unwrap();

        assert_eq!(p3, expected);
    }
}
