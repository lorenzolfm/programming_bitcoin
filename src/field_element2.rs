#[allow(unused)]
#[derive(Debug, PartialEq)]
pub struct FieldElement {
    num: crypto_bigint::U256,
    prime: crypto_bigint::U256,
}

#[allow(unused)]
impl FieldElement {
    pub fn new(num: crypto_bigint::U256, prime: crypto_bigint::U256) -> Result<Self, crate::Error> {
        if num >= prime {
            let prime_minus_one = prime - crypto_bigint::U256::ONE;
            let err = format!("Num {num} not in field range 0 to {prime_minus_one}");
            return Err(crate::Error::ValueError(err));
        }

        Ok(FieldElement { num, prime })
    }

    pub fn add(self, rhs: FieldElement) -> Result<FieldElement, crate::Error> {
        if self.prime != rhs.prime {
            return Err(crate::Error::ValueError(
                "Primes must be the same".to_string(),
            ));
        }

        Ok(FieldElement {
            num: self.num.add_mod(&rhs.num, &self.prime),
            prime: self.prime,
        })
    }

    pub fn sub(self, rhs: FieldElement) -> Result<FieldElement, crate::Error> {
        if self.prime != rhs.prime {
            return Err(crate::Error::ValueError(
                "Primes must be the same".to_string(),
            ));
        }

        Ok(FieldElement {
            num: self.num.sub_mod(&rhs.num, &self.prime),
            prime: self.prime,
        })
    }
}

impl std::fmt::Display for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}
