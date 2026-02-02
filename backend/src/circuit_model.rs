use num::complex::Complex32;
use regex::{Error, Regex};

use crate::elements::Element;

struct CircuitNode<'a> {
    current: Vec<Element>,
    next: Option<Vec<&'a Element>>,
}

impl<'a> CircuitNode<'a> {
    fn current(self) -> Vec<Element> {
        self.current
    }

    fn next(self) -> Option<Vec<&'a Element>> {
        self.next
    }
}

pub struct CircuitModel {
    circuit: String,
    parsed_circuit: Option<Vec<String>>,
}

impl CircuitModel {
    pub fn new(circuit: String) -> Self {
        Self {
            circuit,
            parsed_circuit: None,
        }
    }

    // pub fn with_parsed_circuit(&self, parsed_circuit: Vec<String>) -> Self {
    //     Self {
    //         circuit: self.circuit,
    //         parsed_circuit: Some(parsed_circuit),
    //     }
    // }

    // RR(RC)(RC)
    pub fn parse_circuit(self, circuit: &str) -> () {
        let mut impedance = Complex32 { re: 0., im: 0. };
        let mut depth: u8 = 0;

        fn parse_series(circuit: &str, impedance: Complex32, depth: u8) -> (Complex32, u8) {
            let char = circuit.chars().nth(0);
            if char == Some('(') {
                return (
                    impedance
                        + parse_parallel(&circuit.get(1..).expect("Circuit should be non-empty")),
                    depth + 1,
                );
            } else {
                return (impedance
                    + Element::from_str(
                        &circuit.get(1..).expect("Circuit should be non-empty"),
                        params_complex,
                        params_f32,
                    ));
            }
        }

        let parse_series = |circuit: &str, impedance: &mut Complex32, depth: &mut u8| {};

        fn parse_parallel(circuit: &str) -> Complex32 {
            Complex32 { re: 0., im: 0. }
        }
    }
}
