use std::{
    i128,
    ops::{Add, Sub},
};

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct FieldElement {
    pub num: u128,
    pub prime: u128,
}

#[allow(unused)]
impl FieldElement {
    pub fn new(num: u128, prime: u128) -> Result<Self, crate::Error> {
        if num >= prime {
            return Err(crate::Error::ValueError(format!(
                "Num {num} not in field range 0 to {}",
                prime - 1
            )));
        }

        Ok(FieldElement { num, prime })
    }

    pub fn add(&self, other: Self) -> Result<Self, crate::Error> {
        if self.prime != other.prime {
            return Err(crate::Error::TypeError(
                "Cannot add two numbers in different fields".to_owned(),
            ));
        }

        let num = (self.num.add(other.num)).rem_euclid(self.prime);

        Ok(FieldElement {
            num,
            prime: self.prime,
        })
    }

    pub fn sub(&self, other: Self) -> Result<Self, crate::Error> {
        if self.prime != other.prime {
            return Err(crate::Error::TypeError(
                "Cannot subtract two nubmers in different fields".to_owned(),
            ));
        }

        let a = i128::try_from(self.num).map_err(|e| crate::Error::Conversion(e))?;
        let b = i128::try_from(other.num).map_err(|e| crate::Error::Conversion(e))?;
        let c = i128::try_from(self.prime).map_err(|e| crate::Error::Conversion(e))?;

        let num = (a.sub(b)).rem_euclid(c);
        let num = u128::try_from(num).map_err(|e| crate::Error::Conversion(e))?;

        Ok(FieldElement {
            num,
            prime: self.prime,
        })
    }

    pub fn mul(&self, other: Self) -> Result<Self, crate::Error> {
        let mut result = FieldElement::new(0, self.prime)?;
        let mut count = other.num;

        while count > 0 {
            result = result.add(self.clone())?;
            count -= 1;
        }

        Ok(result)
    }

    pub fn pow(&self, exponent: i32) -> Result<Self, crate::Error> {
        let prime_minus_one =
            i128::try_from(self.prime - 1).map_err(|e| crate::Error::Conversion(e))?;

        let exponent = i128::from(exponent);
        let exponent = exponent.rem_euclid(prime_minus_one);

        let mut counter = 0;
        let mut aux = FieldElement::new(1, self.prime)?;

        while counter < exponent {
            aux = self.mul(aux)?;
            counter += 1;
        }

        Ok(aux)
    }

    pub fn div(&self, other: Self) -> Result<Self, crate::Error> {
        let b = other.pow(-1)?;
        let res = self.mul(b)?;

        Ok(res)
    }
}

impl std::fmt::Display for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FieldElement_{}({})", self.num, self.prime)
    }
}

impl std::cmp::PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}
