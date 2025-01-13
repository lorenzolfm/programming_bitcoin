use std::{
    i128,
    num::TryFromIntError,
    ops::{Add, Sub},
};

#[allow(unused)]
#[derive(Debug)]
pub enum Error {
    ValueError(String),
    TypeError(String),
    Conversion(TryFromIntError),
}

#[allow(unused)]
#[derive(Clone, Debug)]
struct FieldElement {
    pub num: u128,
    pub prime: u128,
}

#[allow(unused)]
impl FieldElement {
    pub fn new(num: u128, prime: u128) -> Result<Self, Error> {
        if num >= prime {
            return Err(Error::ValueError(format!(
                "Num {num} not in field range 0 to {}",
                prime - 1
            )));
        }

        Ok(FieldElement { num, prime })
    }

    pub fn add(&self, other: Self) -> Result<Self, Error> {
        if self.prime != other.prime {
            return Err(Error::TypeError(
                "Cannot add two numbers in different fields".to_owned(),
            ));
        }

        let num = (self.num.add(other.num)).rem_euclid(self.prime);

        Ok(FieldElement {
            num,
            prime: self.prime,
        })
    }

    pub fn sub(&self, other: Self) -> Result<Self, Error> {
        if self.prime != other.prime {
            return Err(Error::TypeError(
                "Cannot subtract two nubmers in different fields".to_owned(),
            ));
        }

        let a = i128::try_from(self.num).map_err(|e| Error::Conversion(e))?;
        let b = i128::try_from(other.num).map_err(|e| Error::Conversion(e))?;
        let c = i128::try_from(self.prime).map_err(|e| Error::Conversion(e))?;

        let num = (a.sub(b)).rem_euclid(c);
        let num = u128::try_from(num).map_err(|e| Error::Conversion(e))?;

        Ok(FieldElement {
            num,
            prime: self.prime,
        })
    }

    pub fn mul(&self, other: Self) -> Result<Self, Error> {
        let mut result = FieldElement::new(0, self.prime)?;
        let mut count = other.num;

        while count > 0 {
            result = result.add(self.clone())?;
            count -= 1;
        }

        Ok(result)
    }

    pub fn pow(&self, exponent: u32) -> Result<Self, Error> {
        let mut counter = 0;
        let mut aux = FieldElement::new(1, self.prime)?;

        while counter < exponent {
            aux = self.mul(aux)?;
            counter += 1;
        }

        Ok(aux)
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

#[cfg(test)]
mod tests {
    use super::FieldElement;

    #[test]
    fn test_eq() {
        let a = FieldElement::new(7, 13).unwrap();
        let b = FieldElement::new(6, 13).unwrap();

        assert_ne!(a, b);
        assert_eq!(a, a);
        assert_eq!(b, b);
    }

    mod chapter_1 {
        use super::FieldElement;

        #[test]
        fn exercise2() {
            let p = 57;

            // 44 + 33
            let a = FieldElement::new(44, p).unwrap();
            let b = FieldElement::new(33, p).unwrap();

            let expected = FieldElement::new(20, p).unwrap();
            let actual = a.add(b).unwrap();

            assert_eq!(actual, expected);

            // 9 - 29
            let a = FieldElement::new(9, p).unwrap();
            let b = FieldElement::new(29, p).unwrap();

            let expected = FieldElement::new(37, p).unwrap();
            let actual = a.sub(b).unwrap();

            assert_eq!(actual, expected);

            // 17 + 42 + 49
            let a = FieldElement::new(17, p).unwrap();
            let b = FieldElement::new(42, p).unwrap();
            let c = FieldElement::new(49, p).unwrap();

            let expected = FieldElement::new(51, p).unwrap();
            let actual = a.add(b).unwrap().add(c).unwrap();

            assert_eq!(actual, expected);

            // 52 - 30 - 38
            let a = FieldElement::new(52, p).unwrap();
            let b = FieldElement::new(30, p).unwrap();
            let c = FieldElement::new(38, p).unwrap();

            let expected = FieldElement::new(41, p).unwrap();
            let actual = a.sub(b).unwrap().sub(c).unwrap();

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_pow() {
            let p = 19;
            let a = FieldElement::new(7, p).unwrap();

            let expected = FieldElement::new(1, p).unwrap();
            let actual = a.pow(3).unwrap();

            assert_eq!(actual, expected);

            let a = FieldElement::new(9, p).unwrap();

            let expected = FieldElement::new(7, p).unwrap();
            let actual = a.pow(12).unwrap();

            assert_eq!(actual, expected);
        }

        #[test]
        fn exercise4() {
            let p = 97;

            // 95*45*31
            let a = FieldElement::new(95, p).unwrap();
            let b = FieldElement::new(45, p).unwrap();
            let c = FieldElement::new(31, p).unwrap();

            let expected = FieldElement::new(23, p).unwrap();
            let actual = a.mul(b).unwrap().mul(c).unwrap();

            assert_eq!(actual, expected);

            // 17*13*19*44
            let a = FieldElement::new(17, p).unwrap();
            let b = FieldElement::new(13, p).unwrap();
            let c = FieldElement::new(19, p).unwrap();
            let d = FieldElement::new(44, p).unwrap();

            let expected = FieldElement::new(68, p).unwrap();
            let actual = a.mul(b).unwrap().mul(c).unwrap().mul(d).unwrap();

            assert_eq!(actual, expected);

            // 12^7 * 77^49
            let a = FieldElement::new(12, p).unwrap();
            let b = FieldElement::new(77, p).unwrap();

            let expected = FieldElement::new(63, p).unwrap();
            let actual = a.pow(7).unwrap().mul(b.pow(49).unwrap()).unwrap();

            assert_eq!(actual, expected);
        }
    }
}
