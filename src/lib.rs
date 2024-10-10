mod field_element;
mod point;

#[derive(Debug)]
pub enum Error {
    ValueError,
    TypeError(String),
    Conversion(std::num::TryFromIntError),
}
