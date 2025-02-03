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
            let a = FieldElement::<57>::new(44);
            let b = FieldElement::<57>::new(33);

            let expected = FieldElement::<57>::new(20);
            let actual = a + b;

            assert_eq!(actual, expected);

            // 9 - 29
            let a = FieldElement::<57>::new(9);
            let b = FieldElement::<57>::new(29);

            let expected = FieldElement::<57>::new(37);
            let actual = a - b;

            assert_eq!(actual, expected);

            // 17 + 42 + 49
            let a = FieldElement::<57>::new(17);
            let b = FieldElement::<57>::new(42);
            let c = FieldElement::<57>::new(49);

            let expected = FieldElement::<57>::new(51);
            let actual = a + b + c;

            assert_eq!(actual, expected);

            // 52 - 30 - 38
            let a = FieldElement::<57>::new(52);
            let b = FieldElement::<57>::new(30);
            let c = FieldElement::<57>::new(38);

            let expected = FieldElement::<57>::new(41);
            let actual = a - b - c;

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_pow() {
            const P: u128 = 19;

            let a = FieldElement::<P>::new(7);
            let expected = FieldElement::<P>::new(1);
            let actual = a.pow(3);

            assert_eq!(actual, expected);

            let a = FieldElement::<P>::new(9);
            let expected = FieldElement::<P>::new(7);
            let actual = a.pow(12);

            assert_eq!(actual, expected);
        }

        #[test]
        fn exercise4() {
            // 95*45*31
            const P: u128 = 97;
            let a = FieldElement::<P>::new(95);
            let b = FieldElement::<P>::new(45);
            let c = FieldElement::<P>::new(31);

            let expected = FieldElement::<P>::new(23);
            let actual = a * b * c;

            assert_eq!(actual, expected);

            // 17*13*19*44
            let a = FieldElement::<P>::new(17);
            let b = FieldElement::<P>::new(13);
            let c = FieldElement::<P>::new(19);
            let d = FieldElement::<P>::new(44);

            let expected = FieldElement::<P>::new(68);
            let actual = a * b * c * d;

            assert_eq!(actual, expected);

            // 12^7 * 77^49
            let a = FieldElement::<P>::new(12);
            let b = FieldElement::<P>::new(77);

            let expected = FieldElement::<P>::new(63);
            let actual = a.pow(7) * b.pow(49);

            assert_eq!(actual, expected);
        }

        #[test]
        fn exercise8() {
            const P: u128 = 31;

            // 3/24
            let a = FieldElement::<P>::new(3);
            let b = FieldElement::<P>::new(24);

            let expected = FieldElement::<P>::new(4);
            let actual = a / b;

            assert_eq!(actual, expected);

            // 17^-3

            let a = FieldElement::<P>::new(17);
            let expected = FieldElement::<P>::new(29);
            let actual = a.pow(-3);

            assert_eq!(actual, expected);

            // 4^-4 * 11

            let a = FieldElement::<P>::new(4);
            let b = FieldElement::<P>::new(11);

            let expected = FieldElement::<P>::new(13);
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

            let actual = Point::new(Some((2, -4)), a, b);
            assert!(actual.is_err());

            let actual = Point::new(Some((-1, -1)), a, b);
            assert!(actual.is_ok());

            let actual = Point::new(Some((18, 77)), a, b);
            assert!(actual.is_ok());

            let actual = Point::new(Some((5, 7)), a, b);
            assert!(actual.is_err());
        }

        #[derive(Debug, PartialEq)]
        struct Curve;
        impl crate::point::Curve for Curve {
            type Scalar = i128;

            const A: Self::Scalar = 5;
            const B: Self::Scalar = 7;
        }

        #[test]
        fn exercise1_again() {
            let p = crate::point::Point::<Curve>::new(Some((-2, -4)));
            assert!(p.is_err());

            let p = crate::point::Point::<Curve>::new(Some((-1, -1)));
            assert!(p.is_ok());

            let p = crate::point::Point::<Curve>::new(Some((18, 77)));
            assert!(p.is_ok());

            let p = crate::point::Point::<Curve>::new(Some((5, 7)));
            assert!(p.is_err());
        }

        #[test]
        fn exercise6_again() {
            let p1 = crate::point::Point::<Curve>::new(Some((-1, -1))).unwrap();
            let p2 = crate::point::Point::<Curve>::new(Some((-1, -1))).unwrap();

            let expected = crate::point::Point::<Curve>::new(Some((18, 77))).unwrap();
            let actual = p1 + p2;

            assert_eq!(actual, expected);
        }

        #[test]
        fn exercise6() {
            let a = 5;
            let b = 7;

            let p1 = Point::new(Some((-1, -1)), a, b).unwrap();
            let p2 = Point::new(Some((-1, -1)), a, b).unwrap();
            let expected = Point::new(Some((18, 77)), a, b).unwrap();
            let actual = p1.add(&p2).unwrap();

            assert_eq!(actual, expected);
        }
    }

    mod chapter3 {
        use crate::{
            field_element::FieldElement,
            point::{Curve, Point},
        };

        const P: u128 = 223;
        const A: FieldElement<P> = FieldElement::<P>::new(0);
        const B: FieldElement<P> = FieldElement::<P>::new(7);

        #[derive(Debug, PartialEq)]
        struct Secp256k1;
        impl Curve for Secp256k1 {
            type Scalar = FieldElement<P>;

            const A: Self::Scalar = A;
            const B: Self::Scalar = B;
        }

        #[test]
        fn ecc_test() {
            let p = Point::<Secp256k1>::new(Some((
                FieldElement::<P>::new(192),
                FieldElement::<P>::new(105),
            )));

            assert!(p.is_ok());

            let p = crate::point::Point::<Secp256k1>::new(Some((
                FieldElement::<P>::new(17),
                FieldElement::<P>::new(56),
            )));

            assert!(p.is_ok());

            let p = crate::point::Point::<Secp256k1>::new(Some((
                FieldElement::<P>::new(1),
                FieldElement::<P>::new(193),
            )));

            assert!(p.is_ok());

            let p = crate::point::Point::<Secp256k1>::new(Some((
                FieldElement::<P>::new(200),
                FieldElement::<P>::new(119),
            )));

            assert!(p.is_err());

            let p = crate::point::Point::<Secp256k1>::new(Some((
                FieldElement::<P>::new(42),
                FieldElement::<P>::new(99),
            )));

            assert!(p.is_err());
        }

        #[test]
        fn exercise3() {
            let p1 = Point::<Secp256k1>::new(Some((
                FieldElement::<P>::new(170),
                FieldElement::<P>::new(142),
            )))
            .unwrap();

            let p2 = Point::<Secp256k1>::new(Some((
                FieldElement::<P>::new(60),
                FieldElement::<P>::new(139),
            )))
            .unwrap();

            let expected = Point::<Secp256k1>::new(Some((
                FieldElement::<P>::new(220),
                FieldElement::<P>::new(181),
            )))
            .unwrap();

            //let actual = p1 + p2;

            //assert_eq!(actual, expected);
        }
    }
}
