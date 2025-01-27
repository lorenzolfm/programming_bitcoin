use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct FieldElement<const P: u128>(u128);

#[allow(unused)]
impl<const P: u128> FieldElement<P> {
    pub fn new(num: u128) -> Result<Self, crate::Error> {
        if num >= P {
            return Err(crate::Error::ValueError(format!(
                "Num {num} not in field range 0 to {}",
                P - 1
            )));
        }

        Ok(FieldElement(num))
    }

    pub fn checked_sub(self, other: FieldElement<P>) -> Option<FieldElement<P>> {
        let a = i128::try_from(*self).ok()?;
        let b = i128::try_from(*other).ok()?;
        let c = i128::try_from(P).ok()?;

        let num = (a.sub(b)).rem_euclid(c);
        let num = u128::try_from(num).ok()?;

        Some(FieldElement(num))
    }
}

impl<const P: u128> std::ops::Deref for FieldElement<P> {
    type Target = u128;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const P: u128> std::ops::Add for FieldElement<P> {
    type Output = FieldElement<P>;

    fn add(self, other: Self) -> Self::Output {
        FieldElement((self.0.add(other.0)).rem_euclid(P))
    }
}

impl<const P: u128> std::ops::Sub for FieldElement<P> {
    type Output = FieldElement<P>;

    fn sub(self, other: Self) -> Self::Output {
        self.checked_sub(other).expect("Conversion error")
    }
}

impl<const P: u128> std::ops::Mul for FieldElement<P> {
    type Output = FieldElement<P>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = FieldElement(0);
        let mut count = *rhs;

        while count > 0 {
            result = result.add(self);
            count -= 1;
        }

        result
    }
}

impl<const P: u128> std::ops::Div for FieldElement<P> {
    type Output = FieldElement<P>;

    fn div(self, rhs: Self) -> Self::Output {
        let b = rhs.pow(-1);
        let res = self * b;

        res
    }
}

impl<const P: u128> FieldElement<P> {
    pub fn pow(self, exponent: i32) -> FieldElement<P> {
        let prime_minus_one = i128::try_from(P - 1).expect("Conversion error");

        let exponent = i128::from(exponent);
        let exponent = exponent.rem_euclid(prime_minus_one);

        let mut counter = 0;
        let mut aux = FieldElement(1);

        while counter < exponent {
            aux = self * aux;
            counter += 1;
        }

        aux
    }
}
