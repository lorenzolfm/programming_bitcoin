use crate::Pow;

pub trait Curve {
    type Scalar;
    const A: Self::Scalar;
    const B: Self::Scalar;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Point<C: Curve> {
    Real(C::Scalar, C::Scalar),
    Infinity,
}

impl<C: Curve> std::fmt::Display for Point<C>
where
    C::Scalar: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Point::Real(x, y) => write!(f, "({x:?}, {y:?})"),
            Point::Infinity => write!(f, "∞"),
        }
    }
}

impl crate::Pow for i128 {
    type Output = Self;

    fn pow(self, exponent: i32) -> Self::Output {
        self.pow(exponent as u32)
    }
}

#[allow(unused)]
impl<C: Curve> Point<C>
where
    C::Scalar: crate::Pow<Output = C::Scalar>
        + std::ops::Add<C::Scalar, Output = C::Scalar>
        + std::ops::Mul<C::Scalar, Output = C::Scalar>
        + std::ops::Sub<C::Scalar, Output = C::Scalar>
        + std::ops::Div<C::Scalar, Output = C::Scalar>
        + Copy
        + PartialEq
        + std::fmt::Display,
{
    pub fn new(point: Option<(C::Scalar, C::Scalar)>) -> Result<Self, crate::Error> {
        match point {
            None => Ok(Point::Infinity),
            Some((x, y)) => {
                if y.pow(2) != x.pow(3) + C::A * x + C::B {
                    return Err(crate::Error::ValueError(format!(
                        "({x}, {y}) is not on the curve"
                    )));
                }

                Ok(Point::Real(x, y))
            }
        }
    }
}

impl<C: Curve + std::cmp::PartialEq> std::ops::Add for Point<C>
where
    C::Scalar: Copy
        + PartialEq
        + From<i128>
        + crate::Pow<Output = C::Scalar>
        + std::ops::Add<C::Scalar, Output = C::Scalar>
        + std::ops::Sub<C::Scalar, Output = C::Scalar>
        + std::ops::Mul<C::Scalar, Output = C::Scalar>
        + std::ops::Sub<C::Scalar, Output = C::Scalar>
        + std::ops::Div<C::Scalar, Output = C::Scalar>,
{
    type Output = Point<C>;

    fn add(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Point::Real(_, _), Point::Infinity) => self,
            (Point::Infinity, Point::Real(_, _)) => rhs,
            (Point::Infinity, Point::Infinity) => Point::Infinity,
            (Point::Real(x, y), Point::Real(other_x, other_y)) => {
                if x == other_x && y != other_y {
                    return Point::Infinity;
                }

                let slope = if self == rhs {
                    let zero = C::Scalar::from(0);

                    if *y == zero {
                        return Point::Infinity;
                    }

                    let three = C::Scalar::from(3);
                    let two = C::Scalar::from(2);

                    (three * (x.pow(2)) + C::A) / (two * *y)
                } else {
                    (*other_y - *y) / (*other_x - *x)
                };

                let new_x = slope.pow(2) - *x - *other_x;
                let new_y = slope * (*x - new_x) - *y;

                Point::Real(new_x, new_y)
            }
        }
    }
}

#[allow(unused)]
impl<C: Curve + std::cmp::PartialEq + Copy> Point<C>
where
    C::Scalar: Copy
        + PartialEq
        + From<i128>
        + crate::Pow<Output = C::Scalar>
        + std::ops::Add<C::Scalar, Output = C::Scalar>
        + std::ops::Sub<C::Scalar, Output = C::Scalar>
        + std::ops::Mul<C::Scalar, Output = C::Scalar>
        + std::ops::Sub<C::Scalar, Output = C::Scalar>
        + std::ops::Div<C::Scalar, Output = C::Scalar>,
{
    pub fn scalar_mul(self, scalar: i32) -> Point<C> {
        let mut coef = scalar;
        let mut current = self;
        let mut result = Point::Infinity;

        while coef > 0 {
            if coef & 1 == 1 {
                result = result + current;
            }
            current = current + current;
            coef = coef >> 1;
        }

        result
    }
}
