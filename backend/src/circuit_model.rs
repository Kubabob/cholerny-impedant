use regex::{Error, Regex};
use std::{char, rc::Rc};

use crate::elements::Element;

struct CircuitNode {
    current: Rc<[Element]>,
    next: Option<Rc<[Element]>>,
}

impl CircuitNode {
    fn current(self) -> Rc<[Element]> {
        self.current.clone()
    }

    fn next(self) -> Option<Rc<[Element]>> {
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
    pub fn parse_circuit(self) -> Result<Vec<String>, Error> {
        let pattern = Regex::new(r"\(.*\)")?;
        let matches: Vec<String> = pattern
            .find_iter(&self.circuit)
            .map(|m| m.as_str().to_string())
            .collect();
        // self.parsed_circuit = Some(matches.clone());
        Ok(matches)
    }
}
