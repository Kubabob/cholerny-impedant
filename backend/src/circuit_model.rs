use std::{collections::HashMap, vec::IntoIter};

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
        let mut idx = 0;
        while idx < self.circuit.len() {
            if self.circuit.get(idx..idx + 1) == Some("(") {
                depth += 1;
                // Add new layer
                if layers.len() <= depth {
                    layers.push(Vec::new());
                }
                idx += 1;
                continue;
            } else if self.circuit.get(idx..idx + 1) == Some(")") {
                // Calculate impedance for current depth
                let layer_impedance: Complex32;
                if depth % 2 == 0 {
                    layer_impedance = layers
                        .get(depth)
                        .expect("This layer should exist {depth}")
                        .iter()
                        .sum();
                } else {
                    layer_impedance = 1.
                        / layers
                            .get(depth)
                            .expect("This layer should exist {depth}")
                            .iter()
                            .map(|impedance| 1. / impedance)
                            .sum::<Complex32>();
                }

                // Add current layers impedance to upper level
                layers
                    .get_mut(depth - 1)
                    .expect("This layer should exist {depth}")
                    .push(layer_impedance);

                // Clear current layer
                layers
                    .get_mut(depth)
                    .expect("This layer should exist {depth}")
                    .clear();

                depth -= 1;
                idx += 1;
                continue;
            } else {
                // Parse element identifier - try longer matches first
                let (element_key, element_len) = Self::parse_element_at(&self.circuit, idx);
                idx += element_len;

                layers
                    .get_mut(depth)
                    .expect(&format!("Hashmap should have layer number {}", depth))
                    .push(
                        self.elements
                            .get_mut(element_key)
                            .expect("Element iterator should exist")
                            .next()
                            .expect("Element from iterator should exist")
                            .impedance(frequency),
                    );
            }
        }

        // Return sum of series part of circuit
        layers.get(0).expect("Layer 0 should exist").iter().sum()
    }

    /// Parses an element identifier at the given position in the circuit string.
    /// Returns a tuple of (element_key, length) where element_key is the matched
    /// element identifier and length is how many characters were consumed.
    fn parse_element_at(circuit: &str, idx: usize) -> (&str, usize) {
        // Try to match multi-character element names first (longest to shortest)
        // 4-character elements: TLMQ, Zarc
        if idx + 4 <= circuit.len() {
            let four_char = &circuit[idx..idx + 4];
            if four_char == "TLMQ" || four_char == "Zarc" {
                return (four_char, 4);
            }
        }

        // 3-character elements: CPE
        if idx + 3 <= circuit.len() {
            let three_char = &circuit[idx..idx + 3];
            if three_char == "CPE" {
                return (three_char, 3);
            }
        }

        // 2-character elements: Wo, Ws, La, Gs
        if idx + 2 <= circuit.len() {
            let two_char = &circuit[idx..idx + 2];
            if two_char == "Wo" || two_char == "Ws" || two_char == "La" || two_char == "Gs" {
                return (two_char, 2);
            }
        }

        // Single-character elements: R, C, L, W, G, K, T
        let one_char = &circuit[idx..idx + 1];
        (one_char, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod impedance {
        use super::*;
        #[test]
        fn case_0() {
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
            let mut elements = HashMap::new();
            let freq = 1.;
            elements.insert(
                "R",
                vec![
                    Element::R { R: 1. },
                    Element::R { R: 2. },
                    Element::R { R: 4. },
                ]
                .into_iter(),
            );

            elements.insert(
                "C",
                vec![Element::C { C: 3. }, Element::C { C: 5. }].into_iter(),
            );

            let mut circuit_model = CircuitModel::new(String::from("R(RC)(RC)"), elements);
            assert_eq!(
                circuit_model.impedance(freq),
                Complex32 {
                    re: 1.0016595,
                    im: -0.08484332
                }
            );
        }

        #[test]
        fn case_2() {
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
            assert_eq!(
                circuit_model.impedance(freq),
                Complex32 {
                    re: 1.8571429,
                    im: 0.
                }
            );
        }

        #[test]
        fn case_4() {
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
                Complex32 {
                    re: 2.4461539,
                    im: 0.
                }
            );
        }

        #[test]
        fn case_7() {
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
                Complex32 {
                    re: 2.5555556,
                    im: 0.
                }
            );
        }

        #[test]
        fn case_8() {
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
                Complex32 {
                    re: 6.2291317,
                    im: 0.
                }
            );
        }

        #[test]
        fn case_9() {
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
                Complex32 {
                    re: 1.9790795,
                    im: 0.
                }
            );
        }

        #[test]
        fn case_10() {
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

        #[test]
        fn case_multi_char_elements() {
            let mut elements = HashMap::new();
            let freq = 1.;

            // Test CPE, R, and C elements
            elements.insert("R", vec![Element::R { R: 100. }].into_iter());
            elements.insert(
                "CPE",
                vec![Element::CPE {
                    Q: 1e-6,
                    alpha: 0.9,
                }]
                .into_iter(),
            );
            elements.insert("C", vec![Element::C { C: 1e-6 }].into_iter());

            // Circuit: R in series with parallel combination of CPE and C
            let mut circuit_model = CircuitModel::new(String::from("R(CPEC)"), elements);
            let result = circuit_model.impedance(freq);

            // Just verify it computes without panicking and returns a complex number
            assert!(result.re.is_finite());
            assert!(result.im.is_finite());
        }

        #[test]
        fn case_warburg_elements() {
            let mut elements = HashMap::new();
            let freq = 1.;

            // Test Wo (open Warburg) and Ws (short Warburg)
            elements.insert("R", vec![Element::R { R: 50. }].into_iter());
            elements.insert("Wo", vec![Element::Wo { Z0: 100., tau: 0.1 }].into_iter());
            elements.insert("Ws", vec![Element::Ws { Z0: 100., tau: 0.1 }].into_iter());

            // Circuit: R in series with Wo, parallel with Ws
            let mut circuit_model = CircuitModel::new(String::from("RWo(Ws)"), elements);
            let result = circuit_model.impedance(freq);

            assert!(result.re.is_finite());
            assert!(result.im.is_finite());
        }

        #[test]
        fn case_complex_multi_char() {
            let mut elements = HashMap::new();
            let freq = 10.;

            // Test TLMQ and Zarc elements
            elements.insert("R", vec![Element::R { R: 10. }].into_iter());
            elements.insert(
                "TLMQ",
                vec![Element::TLMQ {
                    Rion: 50.,
                    Qs: 1e-6,
                    gamma: 0.85,
                }]
                .into_iter(),
            );
            elements.insert(
                "Zarc",
                vec![Element::Zarc {
                    R: 100.,
                    tau_k: 0.01,
                    gamma: 0.9,
                }]
                .into_iter(),
            );

            // Circuit: R in series with parallel TLMQ and Zarc
            let mut circuit_model = CircuitModel::new(String::from("R(TLMQZarc)"), elements);
            let result = circuit_model.impedance(freq);

            assert!(result.re.is_finite());
            assert!(result.im.is_finite());
            assert!(result.re > 0.); // Should have positive real part
        }
    }
}
