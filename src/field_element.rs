use std::{
    i128,
    num::TryFromIntError,
    ops::{Add, Sub},
};

#[derive(Debug)]
pub enum Error {
    ValueError(String),
    TypeError(String),
    Conversion(TryFromIntError),
}

#[derive(Debug)]
struct FieldElement {
    pub num: u128,
    pub prime: u128,
}

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
        fn exercise1() {
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
    }
}
