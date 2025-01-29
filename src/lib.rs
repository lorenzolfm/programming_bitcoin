mod field_element;
mod point;

// TODO: implement ops vs checked_ops (i.e. checked_add vs add)

pub trait Pow {
    type Output;

    fn pow(self, rhs: i32) -> Self::Output;
}

#[allow(unused)]
#[derive(Debug)]
pub enum Error {
    ValueError(String),
    TypeError(String),
    Conversion(std::num::TryFromIntError),
}

#[cfg(test)]
mod tests {
    mod chapter_1 {
        use crate::{field_element::FieldElement, Pow};

        #[test]
        fn exercise2() {
            // 44 + 33
            let a = FieldElement::<57>::new(44).unwrap();
            let b = FieldElement::<57>::new(33).unwrap();

            let expected = FieldElement::<57>::new(20).unwrap();
            let actual = a + b;

            assert_eq!(actual, expected);

            // 9 - 29
            let a = FieldElement::<57>::new(9).unwrap();
            let b = FieldElement::<57>::new(29).unwrap();

            let expected = FieldElement::<57>::new(37).unwrap();
            let actual = a - b;

            assert_eq!(actual, expected);

            // 17 + 42 + 49
            let a = FieldElement::<57>::new(17).unwrap();
            let b = FieldElement::<57>::new(42).unwrap();
            let c = FieldElement::<57>::new(49).unwrap();

            let expected = FieldElement::<57>::new(51).unwrap();
            let actual = a + b + c;

            assert_eq!(actual, expected);

            // 52 - 30 - 38
            let a = FieldElement::<57>::new(52).unwrap();
            let b = FieldElement::<57>::new(30).unwrap();
            let c = FieldElement::<57>::new(38).unwrap();

            let expected = FieldElement::<57>::new(41).unwrap();
            let actual = a - b - c;

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_pow() {
            const P: u128 = 19;

            let a = FieldElement::<P>::new(7).unwrap();
            let expected = FieldElement::<P>::new(1).unwrap();
            let actual = a.pow(3);

            assert_eq!(actual, expected);

            let a = FieldElement::<P>::new(9).unwrap();
            let expected = FieldElement::<P>::new(7).unwrap();
            let actual = a.pow(12);

            assert_eq!(actual, expected);
        }

        #[test]
        fn exercise4() {
            // 95*45*31
            const P: u128 = 97;
            let a = FieldElement::<P>::new(95).unwrap();
            let b = FieldElement::<P>::new(45).unwrap();
            let c = FieldElement::<P>::new(31).unwrap();

            let expected = FieldElement::<P>::new(23).unwrap();
            let actual = a * b * c;

            assert_eq!(actual, expected);

            // 17*13*19*44
            let a = FieldElement::<P>::new(17).unwrap();
            let b = FieldElement::<P>::new(13).unwrap();
            let c = FieldElement::<P>::new(19).unwrap();
            let d = FieldElement::<P>::new(44).unwrap();

            let expected = FieldElement::<P>::new(68).unwrap();
            let actual = a * b * c * d;

            assert_eq!(actual, expected);

            // 12^7 * 77^49
            let a = FieldElement::<P>::new(12).unwrap();
            let b = FieldElement::<P>::new(77).unwrap();

            let expected = FieldElement::<P>::new(63).unwrap();
            let actual = a.pow(7) * b.pow(49);

            assert_eq!(actual, expected);
        }

        #[test]
        fn exercise8() {
            const P: u128 = 31;

            // 3/24
            let a = FieldElement::<P>::new(3).unwrap();
            let b = FieldElement::<P>::new(24).unwrap();

            let expected = FieldElement::<P>::new(4).unwrap();
            let actual = a / b;

            assert_eq!(actual, expected);

            // 17^-3

            let a = FieldElement::<P>::new(17).unwrap();
            let expected = FieldElement::<P>::new(29).unwrap();
            let actual = a.pow(-3);

            assert_eq!(actual, expected);

            // 4^-4 * 11

            let a = FieldElement::<P>::new(4).unwrap();
            let b = FieldElement::<P>::new(11).unwrap();

            let expected = FieldElement::<P>::new(13).unwrap();
            let actual = a.pow(-4) * b;

            assert_eq!(actual, expected);
        }
    }

    mod chapter2 {
        use crate::point::{Add, Point};

        #[test]
        fn exercise1() {
            let a = 5;
            let b = 7;

            let actual = Point::new(Some(2), Some(-4), a, b);
            assert!(actual.is_err());

            let actual = Point::new(Some(-1), Some(-1), a, b);
            assert!(actual.is_ok());

            let actual = Point::new(Some(18), Some(77), a, b);
            assert!(actual.is_ok());

            let actual = Point::new(Some(5), Some(7), a, b);
            assert!(actual.is_err());
        }

        #[test]
        fn exercise6() {
            let a = 5;
            let b = 7;

            let p1 = Point::new(Some(-1), Some(-1), a, b).unwrap();
            let p2 = Point::new(Some(-1), Some(-1), a, b).unwrap();
            let expected = Point::new(Some(18), Some(77), a, b).unwrap();
            let actual = p1.add(&p2).unwrap();

            assert_eq!(actual, expected);
        }
    }

    /*
    mod chapter_2 {

        #[test]
        fn exercise6() {
            let a = 5;
            let b = 7;

            let p1 = Point::new(Some(-1), Some(-1), a, b).unwrap();
            let p2 = Point::new(Some(-1), Some(-1), a, b).unwrap();
            let expected = Point::new(Some(18), Some(77), a, b).unwrap();
            let actual = p1.add(&p2).unwrap();

            assert_eq!(actual, expected);
        }
    }
    */
}
