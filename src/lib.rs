mod field_element;
mod point;

// TODO: implement ops vs checked_ops (i.e. checked_add vs add)

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
        use crate::field_element::FieldElement;

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

        #[test]
        fn exercise8() {
            let p = 31;

            // 3/24
            let a = FieldElement::new(3, p).unwrap();
            let b = FieldElement::new(24, p).unwrap();

            let expected = FieldElement::new(4, p).unwrap();
            let actual = a.div(b).unwrap();

            assert_eq!(actual, expected);

            // 17^-3
            let a = FieldElement::new(17, p).unwrap();

            let expected = FieldElement::new(29, p).unwrap();
            let actual = a.pow(-3).unwrap();

            assert_eq!(actual, expected);

            // 4^-4 * 11
            let a = FieldElement::new(4, p).unwrap();
            let b = FieldElement::new(11, p).unwrap();

            let expected = FieldElement::new(13, p).unwrap();
            let actual = a.pow(-4).unwrap().mul(b).unwrap();

            assert_eq!(actual, expected);
        }
    }

    mod chapter_2 {
        use crate::point::Point;

        #[test]
        fn exercise1() {
            let a = 5;
            let b = 7;

            let x = 2;
            let y = -4;
            let actual = Point::new(Some(x), Some(y), a, b);

            assert!(actual.is_err());

            let x = -1;
            let y = -1;
            let actual = Point::new(Some(x), Some(y), a, b);

            assert!(actual.is_ok());

            let x = 18;
            let y = 77;
            let actual = Point::new(Some(x), Some(y), a, b);

            assert!(actual.is_ok());

            let x = 5;
            let y = 7;
            let actual = Point::new(Some(x), Some(y), a, b);

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
}
