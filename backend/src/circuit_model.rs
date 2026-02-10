use std::{
    collections::{HashMap, vec_deque::Iter},
    vec::IntoIter,
};

use num::complex::Complex32;

use crate::elements::Element;

pub struct CircuitModel<'a> {
    circuit: String,
    elements: HashMap<&'a str, IntoIter<Element>>,
}

impl<'a> CircuitModel<'a> {
    pub fn new(circuit: String, elements: HashMap<&'a str, IntoIter<Element>>) -> Self {
        Self { circuit, elements }
    }

    pub fn impedance(&mut self, frequency: f32) -> Complex32 {
        let mut layers: Vec<Vec<Complex32>> = vec![Vec::new()];
        let mut depth: usize = 0;

        // Create vector with vector of impedances
        for idx in 0..self.circuit.len() {
            if self.circuit.get(idx..idx + 1) == Some("(") {
                depth += 1;
                layers.push(Vec::new());
                continue;
            } else if self.circuit.get(idx..idx + 1) == Some(")") {
                depth -= 1;
                continue;
            } else {
                layers
                    .get_mut(depth)
                    .expect(&format!("Hashmap should have layer number {}", depth))
                    .push(
                        self.elements
                            .get_mut(
                                self.circuit
                                    .get(idx..idx + 1)
                                    .expect("Circuit should be long enough"),
                            )
                            .expect("Element iterator should exist")
                            .next()
                            .expect("Element from iterator should exist")
                            .impedance(frequency),
                    );
            }
        }

        // Calculate impedances for parallel parts of circuit
        for layer_idx in (1..layers.len()).rev() {
            let layer_impedance: Complex32 = 1.
                / layers
                    .get(layer_idx)
                    .expect("This layer should exist {layer_idx}")
                    .iter()
                    .map(|impedance| 1. / impedance)
                    .sum::<Complex32>();

            layers
                .get_mut(layer_idx - 1)
                .expect("This layer should exist {layer_idx}")
                .push(layer_impedance);
        }

        // Return sum of series part of circuit
        layers
            .get(0)
            .expect("Layer 0 should exist")
            .iter()
            .fold(Complex32 { re: 0., im: 0. }, |acc, impedance| {
                acc + impedance
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let i = Complex32 { re: 0., im: 1. };
        let mut elements = HashMap::new();
        elements.insert(
            "R",
            vec![
                Element::R { R: 1. },
                Element::R { R: 1. },
                Element::R { R: 1. },
            ]
            .into_iter(),
        );

        let mut circuit_model = CircuitModel::new(String::from("R(RR)"), elements);
        assert_eq!(circuit_model.impedance(1.), Complex32 { re: 1.5, im: 0. });
    }

    #[test]
    fn case_1() {
        let i = Complex32 { re: 0., im: 1. };
        let mut elements = HashMap::new();
        let freq = 1.;
        elements.insert(
            "R",
            vec![
                Element::R { R: 0.223 },
                Element::R { R: 1.519 },
                Element::R { R: 3.507 },
            ]
            .into_iter(),
        );

        elements.insert(
            "C",
            vec![Element::C { C: 0.0025 }, Element::C { C: 0.0814 }].into_iter(),
        );

        let mut circuit_model = CircuitModel::new(String::from("R(RC)(RC)"), elements);
        assert_eq!(
            circuit_model.impedance(freq),
            Complex32 {
                re: 0.19,
                im: -0.019
            }
        );
    }

    #[test]
    fn case_2() {
        let i = Complex32 { re: 0., im: 1. };
        let mut elements = HashMap::new();
        let freq = 1.;
        elements.insert(
            "R",
            vec![
                Element::R { R: 1. },
                Element::R { R: 2. },
                Element::R { R: 2. },
                Element::R { R: 3. },
                Element::R { R: 3. },
            ]
            .into_iter(),
        );

        let mut circuit_model = CircuitModel::new(String::from("R(RR)(RR)"), elements);
        assert_eq!(circuit_model.impedance(freq), Complex32 { re: 3.5, im: 0. });
    }

    #[test]
    fn case_3() {
        let i = Complex32 { re: 0., im: 1. };
        let mut elements = HashMap::new();
        let freq = 1.;
        elements.insert(
            "R",
            vec![
                Element::R { R: 1. },
                Element::R { R: 2. },
                Element::R { R: 2. },
                Element::R { R: 3. },
                Element::R { R: 3. },
            ]
            .into_iter(),
        );

        let mut circuit_model = CircuitModel::new(String::from("R(RR(RR))"), elements);
        assert_eq!(circuit_model.impedance(freq), Complex32 { re: 1.4, im: 0. });
    }

    #[test]
    fn case_4() {
        let i = Complex32 { re: 0., im: 1. };
        let mut elements = HashMap::new();
        let freq = 1.;
        elements.insert(
            "R",
            vec![
                Element::R { R: 1. },
                Element::R { R: 2. },
                Element::R { R: 3. },
            ]
            .into_iter(),
        );

        let mut circuit_model = CircuitModel::new(String::from("R(R)(R)"), elements);
        assert_eq!(circuit_model.impedance(freq), Complex32 { re: 6., im: 0. });
    }

    #[test]
    fn case_5() {
        let i = Complex32 { re: 0., im: 1. };
        let mut elements = HashMap::new();
        let freq = 1.;
        elements.insert(
            "R",
            vec![
                Element::R { R: 1. },
                Element::R { R: 2. },
                Element::R { R: 3. },
            ]
            .into_iter(),
        );

        let mut circuit_model = CircuitModel::new(String::from("(RRR)"), elements);
        assert_eq!(
            circuit_model.impedance(freq),
            Complex32 {
                re: 0.54545456,
                im: 0.
            }
        );
    }

    #[test]
    fn case_6() {
        let i = Complex32 { re: 0., im: 1. };
        let mut elements = HashMap::new();
        let freq = 1.;
        elements.insert(
            "R",
            vec![
                Element::R { R: 1. },
                Element::R { R: 2. },
                Element::R { R: 3. },
                Element::R { R: 4. },
                Element::R { R: 5. },
            ]
            .into_iter(),
        );

        let mut circuit_model = CircuitModel::new(String::from("R(R(R(RR)))"), elements);
        assert_eq!(
            circuit_model.impedance(freq),
            Complex32 { re: 2.45, im: 0. }
        );
    }

    #[test]
    fn case_7() {
        let i = Complex32 { re: 0., im: 1. };
        let mut elements = HashMap::new();
        let freq = 1.;
        elements.insert(
            "R",
            vec![
                Element::R { R: 1. },
                Element::R { R: 2. },
                Element::R { R: 3. },
                Element::R { R: 4. },
            ]
            .into_iter(),
        );

        let mut circuit_model = CircuitModel::new(String::from("R(R(RR))"), elements);
        assert_eq!(
            circuit_model.impedance(freq),
            Complex32 { re: 2.56, im: 0. }
        );
    }

    #[test]
    fn case_8() {
        let i = Complex32 { re: 0., im: 1. };
        let mut elements = HashMap::new();
        let freq = 1.;
        elements.insert(
            "R",
            vec![
                Element::R { R: 1. },
                Element::R { R: 2. },
                Element::R { R: 3. },
                Element::R { R: 4. },
                Element::R { R: 5. },
                Element::R { R: 6. },
                Element::R { R: 7. },
                Element::R { R: 8. },
            ]
            .into_iter(),
        );

        let mut circuit_model = CircuitModel::new(String::from("R(R(R(R(RR))))(RR)"), elements);
        assert_eq!(
            circuit_model.impedance(freq),
            Complex32 { re: 6.23, im: 0. }
        );
    }

    #[test]
    fn case_9() {
        let i = Complex32 { re: 0., im: 1. };
        let mut elements = HashMap::new();
        let freq = 1.;
        elements.insert(
            "R",
            vec![
                Element::R { R: 1. },
                Element::R { R: 2. },
                Element::R { R: 3. },
                Element::R { R: 4. },
                Element::R { R: 5. },
                Element::R { R: 6. },
                Element::R { R: 7. },
            ]
            .into_iter(),
        );

        let mut circuit_model = CircuitModel::new(String::from("R(RR(RR)(RR))"), elements);
        assert_eq!(
            circuit_model.impedance(freq),
            Complex32 { re: 1.98, im: 0. }
        );
    }

    #[test]
    fn case_10() {
        let i = Complex32 { re: 0., im: 1. };
        let mut elements = HashMap::new();
        let freq = 1.;
        elements.insert(
            "R",
            vec![
                Element::R { R: 1. },
                Element::R { R: 2. },
                Element::R { R: 3. },
                Element::R { R: 4. },
                Element::R { R: 5. },
            ]
            .into_iter(),
        );

        let mut circuit_model = CircuitModel::new(String::from("R(RR)(RR)"), elements);
        assert_eq!(
            circuit_model.impedance(freq),
            Complex32 {
                re: 4.422222,
                im: 0.
            }
        );
    }
}
