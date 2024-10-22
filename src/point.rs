use std::ops::Mul;

use crate::{field_element::FieldElement, Error};

#[derive(Copy, Clone)]
pub enum Coordinate<T> {
    Real { x: T, y: T },
    Infinity,
}

#[derive(Copy, Clone)]
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

#[allow(unused)]
impl Point<FieldElement> {
    fn new(
        x: Option<FieldElement>,
        y: Option<FieldElement>,
        a: FieldElement,
        b: FieldElement,
    ) -> Result<Self, Error> {
        match (x, y) {
            (Some(x_val), Some(y_val)) => {
                if y_val.pow(2)? != x_val.pow(3)?.add(a.mul(x_val)?.add(b)?)? {
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
                    let prime = y.prime;
                    let zero = FieldElement::new(0, prime)?;

                    if y == zero {
                        return Ok(Point {
                            a,
                            b,
                            coord: Coordinate::Infinity,
                        });
                    }

                    let three = FieldElement::new(3, prime)?;
                    let two = FieldElement::new(2, prime)?;

                    three.mul(x.pow(2)?)?.add(a)?.div(two.mul(y)?)?
                } else {
                    (other_y.sub(y)?).div((other_x.sub(x)?))?
                };

                let new_x = (slope.pow(2)?).sub(x)?.sub(other_x)?;
                let new_y = slope.mul(x.sub(new_x)?)?.sub(y)?;

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
impl std::fmt::Debug for Point<FieldElement> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x, y) = match self.coord {
            Coordinate::Real { x, y } => (x, y),
            Coordinate::Infinity => todo!(),
        };
        write!(
            f,
            "({}, {}), curve y² = x³+ {}x + {} over F{}",
            x.num, y.num, self.a.num, self.b.num, x.prime
        )
    }
}

impl std::fmt::Debug for Point<i64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x, y) = match self.coord {
            Coordinate::Real { x, y } => (x, y),
            Coordinate::Infinity => todo!(),
        };
        write!(f, "({}, {}), curve y² = x³+ {}x + {}", x, y, self.a, self.b)
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
    use crate::{field_element::FieldElement, point::Point};

    #[test]
    fn point() {
        let a = 5;
        let b = 7;

        let p1 = Point::<i64>::new(Some(2), Some(4), a, b);
        assert!(p1.is_err());

        let p2 = Point::<i64>::new(Some(-1), Some(-1), a, b);
        assert!(p2.is_ok());

        let p3 = Point::<i64>::new(Some(18), Some(77), a, b);
        assert!(p3.is_ok());

        let p4 = Point::<i64>::new(Some(5), Some(7), a, b);
        assert!(p4.is_err());
    }

    #[test]
    fn point_on_field_element() {
        let p = 223;
        let a = FieldElement::new(0, p).unwrap();
        let b = FieldElement::new(7, p).unwrap();
        let x = FieldElement::new(192, p).unwrap();
        let y = FieldElement::new(105, p).unwrap();

        let point = Point::<FieldElement>::new(Some(x), Some(y), a, b);
        assert!(point.is_ok());

        let x = FieldElement::new(17, p).unwrap();
        let y = FieldElement::new(56, p).unwrap();

        let point = Point::<FieldElement>::new(Some(x), Some(y), a, b);
        assert!(point.is_ok());

        let x = FieldElement::new(200, p).unwrap();
        let y = FieldElement::new(119, p).unwrap();

        let point = Point::<FieldElement>::new(Some(x), Some(y), a, b);
        assert!(point.is_err());

        let x = FieldElement::new(1, p).unwrap();
        let y = FieldElement::new(193, p).unwrap();

        let point = Point::<FieldElement>::new(Some(x), Some(y), a, b);
        assert!(point.is_ok());

        let x = FieldElement::new(42, p).unwrap();
        let y = FieldElement::new(99, p).unwrap();

        let point = Point::<FieldElement>::new(Some(x), Some(y), a, b);
        assert!(point.is_err());
    }

    #[test]
    fn add() {
        let a = 5;
        let b = 7;

        let p1 = Point::<i64>::new(Some(2), Some(5), a, b).unwrap();
        let p2 = Point::<i64>::new(Some(-1), Some(-1), a, b).unwrap();
        let p3 = p1.add(&p2).unwrap();

        let expected = Point::<i64>::new(Some(3), Some(-7), a, b).unwrap();

        assert_eq!(p3, expected);

        let p1 = Point::<i64>::new(Some(-1), Some(-1), a, b).unwrap();
        let p3 = p1.add(&p1).unwrap();

        let expected = Point::<i64>::new(Some(18), Some(77), a, b).unwrap();

        assert_eq!(p3, expected);
    }

    #[test]
    fn add_for_field_element() {
        let p = 223;
        let a = FieldElement::new(0, p).unwrap();
        let b = FieldElement::new(7, p).unwrap();

        let x1 = FieldElement::new(170, p).unwrap();
        let y1 = FieldElement::new(142, p).unwrap();
        let p1 = Point::<FieldElement>::new(Some(x1), Some(y1), a, b).unwrap();

        let x2 = FieldElement::new(60, p).unwrap();
        let y2 = FieldElement::new(139, p).unwrap();
        let p2 = Point::<FieldElement>::new(Some(x2), Some(y2), a, b).unwrap();

        let p3 = p1.add(&p2).unwrap();

        let actual_x = FieldElement::new(220, p).unwrap();
        let actual_y = FieldElement::new(181, p).unwrap();
        let actual = Point::<FieldElement>::new(Some(actual_x), Some(actual_y), a, b).unwrap();

        assert_eq!(p3, actual);

        let x1 = FieldElement::new(47, p).unwrap();
        let y1 = FieldElement::new(71, p).unwrap();
        let p1 = Point::<FieldElement>::new(Some(x1), Some(y1), a, b).unwrap();

        let x2 = FieldElement::new(17, p).unwrap();
        let y2 = FieldElement::new(56, p).unwrap();
        let p2 = Point::<FieldElement>::new(Some(x2), Some(y2), a, b).unwrap();

        let p3 = p1.add(&p2).unwrap();

        let actual_x = FieldElement::new(215, p).unwrap();
        let actual_y = FieldElement::new(68, p).unwrap();
        let actual = Point::<FieldElement>::new(Some(actual_x), Some(actual_y), a, b).unwrap();

        assert_eq!(p3, actual);

        let x1 = FieldElement::new(143, p).unwrap();
        let y1 = FieldElement::new(98, p).unwrap();
        let p1 = Point::<FieldElement>::new(Some(x1), Some(y1), a, b).unwrap();

        let x2 = FieldElement::new(76, p).unwrap();
        let y2 = FieldElement::new(66, p).unwrap();
        let p2 = Point::<FieldElement>::new(Some(x2), Some(y2), a, b).unwrap();

        let p3 = p1.add(&p2).unwrap();

        let actual_x = FieldElement::new(47, p).unwrap();
        let actual_y = FieldElement::new(71, p).unwrap();
        let actual = Point::<FieldElement>::new(Some(actual_x), Some(actual_y), a, b).unwrap();

        assert_eq!(p3, actual);
    }
}
