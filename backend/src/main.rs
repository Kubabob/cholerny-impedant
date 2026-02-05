use num::complex::Complex32;
use regex::Error;

use crate::elements::Element;

mod circuit_model;
mod elements;
mod impedance_data;
mod utils;

fn main() -> Result<(), Error> {
    let v = vec![Element::from_str("R", &[Complex32 { re: 1., im: 0. }], &[])];
    let i = v.into_iter();
    Ok(())
}
