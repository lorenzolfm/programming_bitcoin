#[derive(Debug)]
pub enum Error {
    ValueError,
    TypeError(String),
    Conversion(std::num::TryFromIntError),
}

#[derive(Debug, Clone)]
struct FieldElement {
    num: u64,
    prime: u64,
}

#[allow(unused)]
impl FieldElement {
    fn new(num: u64, prime: u64) -> Result<FieldElement, Error> {
        if num >= prime {
            return Err(Error::ValueError);
        }

        Ok(FieldElement { num, prime })
    }

    fn add(self, rhs: FieldElement) -> Result<Self, Error> {
        if self.prime != rhs.prime {
            return Err(Error::TypeError(
                "Cannot add two numbers in different fields".to_string(),
            ));
        }

        let num = (self.num + rhs.num).rem_euclid(self.prime);

        FieldElement::new(num, self.prime)
    }

    fn sub(self, rhs: FieldElement) -> Result<Self, Error> {
        if self.prime != rhs.prime {
            return Err(Error::TypeError(
                "Cannot subtract numbers in different fields".to_string(),
            ));
        }

        let a_num = i64::try_from(self.num).map_err(|e| Error::Conversion(e))?;
        let b_num = i64::try_from(rhs.num).map_err(|e| Error::Conversion(e))?;
        let c_num = i64::try_from(self.prime).map_err(|e| Error::Conversion(e))?;

        let num = (a_num as i64 - b_num as i64).rem_euclid(c_num);
        let num = u64::try_from(num).map_err(|e| Error::Conversion(e))?;

        FieldElement::new(num, self.prime)
    }

    fn mul(self, rhs: FieldElement) -> Result<Self, Error> {
        let mut result = FieldElement::new(0, self.prime)?;
        let mut count = rhs.num;

        while count > 0 {
            result = result.add(self.clone())?;
            count -= 1;
        }

        Ok(result)
    }

    fn pow(self, exponent: u64) -> Result<Self, Error> {
        let result = (self.num.pow(exponent as u32)).rem_euclid(self.prime);

        Ok(FieldElement::new(result, self.prime)?)
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
    use super::*;

    #[test]
    fn eq() {
        let a = FieldElement::new(7, 13).unwrap();
        let b = FieldElement::new(6, 13).unwrap();

        assert!(a != b);
        assert!(a == a);
    }

    #[test]
    fn add() {
        let p = 57;

        let a = FieldElement::new(44, p).unwrap();
        let b = FieldElement::new(33, p).unwrap();

        assert_eq!(a.add(b).unwrap(), FieldElement::new(20, p).unwrap());

        let a = FieldElement::new(9, p).unwrap();
        let b = FieldElement::new(29, p).unwrap();

        assert_eq!(a.sub(b).unwrap(), FieldElement::new(37, p).unwrap());

        let a = FieldElement::new(17, p).unwrap();
        let b = FieldElement::new(42, p).unwrap();
        let c = FieldElement::new(49, p).unwrap();

        let actual = a.add(b).unwrap().add(c).unwrap();
        assert_eq!(actual, FieldElement::new(51, p).unwrap());
    }

    #[test]
    fn sub() {
        let p = 57;

        let a = FieldElement::new(52, p).unwrap();
        let b = FieldElement::new(30, p).unwrap();
        let c = FieldElement::new(38, p).unwrap();

        let actual = a.sub(b).unwrap().sub(c).unwrap();

        assert_eq!(actual, FieldElement::new(41, p).unwrap());
    }

    #[test]
    fn mul() {
        let p = 97;
        let a = FieldElement::new(95, p).unwrap();
        let b = FieldElement::new(45, p).unwrap();
        let c = FieldElement::new(31, p).unwrap();

        let actual = a.mul(b).unwrap().mul(c).unwrap();

        assert_eq!(actual, FieldElement::new(23, p).unwrap());

        let a = FieldElement::new(17, p).unwrap();
        let b = FieldElement::new(13, p).unwrap();
        let c = FieldElement::new(19, p).unwrap();
        let d = FieldElement::new(44, p).unwrap();

        let actual = a.mul(b).unwrap().mul(c).unwrap().mul(d).unwrap();

        assert_eq!(actual, FieldElement::new(68, p).unwrap());
    }

    #[test]
    fn pow() {
        let p = 19;
        let a = FieldElement::new(7, p).unwrap();

        let actual = a.pow(3).unwrap();

        assert_eq!(actual, FieldElement::new(1, p).unwrap());

        let a = FieldElement::new(9, p).unwrap();

        let actual = a.pow(12).unwrap();

        assert_eq!(actual, FieldElement::new(7, p).unwrap());
    }
}
