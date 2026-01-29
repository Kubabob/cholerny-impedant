use regex::Error;

use crate::circuit_model::CircuitModel;

mod circuit_model;
mod elements;
mod impedance_data;
mod utils;

fn main() -> Result<(), Error> {
    let circuit_schema = String::from("RC(RC)(RC)");
    let circuit_model = CircuitModel::new(circuit_schema);
    let parsed_schema = circuit_model.parse_circuit()?;
    println!("Parsed schema: {parsed_schema:?}");
    // for element in parsed_schema {
    //     println!("{element}");
    // }
    Ok(())
}
