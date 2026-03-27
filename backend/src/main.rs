use num::complex::Complex32;
use regex::Error;

use crate::elements::Element;

mod circuit_fitter;
mod circuit_model;
mod elements;
mod impedance_data;
mod utils;

fn main() -> Result<(), Error> {
    let v = vec![Element::from_str("R", &[1.])];
    let i = v.into_iter();
    Ok(())
}
