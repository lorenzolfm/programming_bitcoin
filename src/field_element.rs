#[derive(Debug)]
pub enum Error {
    ValueError(String),
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
}
