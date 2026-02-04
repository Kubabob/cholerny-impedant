use regex::Error;

mod circuit_model;
mod elements;
mod impedance_data;
mod utils;

fn main() -> Result<(), Error> {
    let circuit = "C)";
    let compared_char = circuit.chars().next();
    println!("{}", compared_char.unwrap());
    Ok(())
}
