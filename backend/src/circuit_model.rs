use std::collections::HashMap;

use num::{Complex, complex::Complex32};

use crate::elements::Element;

pub struct CircuitModel {
    circuit: String,
}

impl CircuitModel {
    pub fn new(circuit: String) -> Self {
        Self { circuit }
    }

    pub fn impedance(
        &self,
        params_complex: &HashMap<&str, &[Complex32]>,
        params_f32: &HashMap<&str, &[f32]>,
        frequency: f32,
    ) -> Complex32 {
        let depth = 0;
        let mut element_counter: HashMap<&str, usize> = HashMap::new();

        fn impedance_series<'a>(
            previous_impedance: Complex32,
            circuit: &'a str,
            depth: i32,
            params_complex: &HashMap<&str, &[Complex32]>,
            params_f32: &HashMap<&str, &[f32]>,
            frequency: f32,
            element_counter: &mut HashMap<&'a str, usize>,
        ) -> Complex32 {
            let compared_char = circuit.chars().next();
            let mut previous_impedance = previous_impedance;

            if compared_char == Some(')') {
                previous_impedance += impedance_series(
                    previous_impedance,
                    &circuit[1..],
                    depth,
                    params_complex,
                    params_f32,
                    frequency,
                    element_counter,
                );
                return previous_impedance;
            } else if compared_char == Some('(') {
                previous_impedance += impedance_parallel(
                    previous_impedance,
                    &circuit[1..],
                    depth,
                    params_complex,
                    params_f32,
                    frequency,
                    element_counter,
                );
                return previous_impedance;
            } else if compared_char == None {
                return previous_impedance;
            } else {
                if element_counter.contains_key(&circuit[..1]) {
                    element_counter.insert(
                        &circuit[..1],
                        element_counter
                            .get(&circuit[..1])
                            .expect("Key should exist in element_counter")
                            + 1,
                    );
                } else {
                    element_counter.insert(&circuit[..1], 0);
                }

                previous_impedance += (impedance_series(
                    previous_impedance,
                    &circuit[1..],
                    depth,
                    params_complex,
                    params_f32,
                    frequency,
                    element_counter,
                ) + Element::from_str(
                    &circuit[..1],
                    params_complex
                        .get(
                            format!(
                                "{}{}",
                                &circuit[..1],
                                &element_counter
                                    .get(&circuit[..1])
                                    .expect("element_counter should have this key")
                            )
                            .as_str(),
                        )
                        .expect("params_complex should contain requested key"),
                    params_f32
                        .get(
                            format!(
                                "{}{}",
                                &circuit[..1],
                                &element_counter
                                    .get(&circuit[..1])
                                    .expect("element_counter should have this key")
                            )
                            .as_str(),
                        )
                        .expect("params_f32 should contain requested key"),
                )
                .impedance(frequency));
                return previous_impedance;
            };
        }

        fn impedance_parallel(
            previous_impedance: Complex32,
            circuit: &str,
            mut depth: i32,
            params_complex: &HashMap<&str, &[Complex32]>,
            params_f32: &HashMap<&str, &[f32]>,
            frequency: f32,
            element_counter: &HashMap<&str, usize>,
        ) -> Complex32 {
            let compared_char = circuit.chars().next();
            let previous_impedance = previous_impedance;

            if compared_char == Some(')') && (depth == 1 || depth == 0) {
                depth -= 1;
                return previous_impedance
                    + Complex32 { re: 1., im: 0. }
                        / (impedance_parallel(
                            previous_impedance,
                            &circuit[1..],
                            depth,
                            params_complex,
                            params_f32,
                            frequency,
                            element_counter,
                        ));
            } else if compared_char == Some('(') {
                depth += 1;
                return previous_impedance
                    + impedance_parallel(
                        previous_impedance,
                        &circuit[1..],
                        depth,
                        params_complex,
                        params_f32,
                        frequency,
                        element_counter,
                    );
            } else if compared_char == None {
                return previous_impedance;
            } else {
                return previous_impedance
                    + impedance_parallel(
                        previous_impedance,
                        &circuit[1..],
                        depth,
                        params_complex,
                        params_f32,
                        frequency,
                        element_counter,
                    )
                    + Complex32 { re: 1., im: 0. }
                        / Element::from_str(
                            &circuit[..1],
                            params_complex
                                .get(
                                    format!(
                                        "{}{}",
                                        &circuit[..1],
                                        element_counter
                                            .get(&circuit[..1])
                                            .expect("element_counter should have this key")
                                    )
                                    .as_str(),
                                )
                                .expect("params_complex should contain requested key"),
                            params_f32
                                .get(
                                    format!(
                                        "{}{}",
                                        &circuit[..1],
                                        element_counter
                                            .get(&circuit[..1])
                                            .expect("element_counter should have this key")
                                    )
                                    .as_str(),
                                )
                                .expect("params_f32 should contain requested key"),
                        )
                        .impedance(frequency);
            }
        }

        return impedance_series(
            Complex { re: 0., im: 0. },
            &self.circuit,
            depth,
            params_complex,
            params_f32,
            frequency,
            &mut element_counter,
        );
    }

    // RR(RC)(RC)
    // pub fn impedance2(
    //     &self,
    //     previous_impedance: Complex32,
    //     circuit: &str,
    //     params_complex: &HashMap<&str, &[Complex32]>,
    //     params_f32: &HashMap<&str, &[f32]>,
    //     frequency: f32,
    // ) -> Complex32 {
    //     let compared_char = circuit.chars().next();
    //     let mut depth = 0;
    //     let mut element_counter: HashMap<&str, usize> = HashMap::new();

    //     if compared_char == Some(')') {
    //         return self.impedance(
    //             previous_impedance,
    //             &circuit[1..],
    //             params_complex,
    //             params_f32,
    //             frequency,
    //         ) + previous_impedance;
    //     } else if compared_char == Some('(') {
    //         return previous_impedance
    //             + inner_impedance(
    //                 previous_impedance,
    //                 &circuit[1..],
    //                 depth,
    //                 params_complex,
    //                 params_f32,
    //                 frequency,
    //                 &element_counter,
    //             );
    //     } else if compared_char == None {
    //         return previous_impedance;
    //     } else {
    //         if element_counter.contains_key(&circuit[..1]) {
    //             element_counter.insert(
    //                 &circuit[..1],
    //                 *element_counter
    //                     .get(&circuit[..1])
    //                     .expect("Key should exist in element_counter")
    //                     + 1,
    //             )
    //         } else {
    //             element_counter.insert(&circuit[..1], 0)
    //         };

    //         return previous_impedance
    //             + self.impedance(
    //                 previous_impedance,
    //                 &circuit[1..],
    //                 params_complex,
    //                 params_f32,
    //                 frequency,
    //             )
    //             + Element::from_str(
    //                 &circuit[..1],
    //                 params_complex
    //                     .get(
    //                         format!(
    //                             "{}{}",
    //                             &circuit[..1],
    //                             element_counter
    //                                 .get(&circuit[..1])
    //                                 .expect("element_counter should have this key")
    //                         )
    //                         .as_str(),
    //                     )
    //                     .expect("params_complex should contain requested key"),
    //                 params_f32
    //                     .get(
    //                         format!(
    //                             "{}{}",
    //                             &circuit[..1],
    //                             element_counter
    //                                 .get(&circuit[..1])
    //                                 .expect("element_counter should have this key")
    //                         )
    //                         .as_str(),
    //                     )
    //                     .expect("params_f32 should contain requested key"),
    //             )
    //             .impedance(frequency);
    //     }

    //     fn inner_impedance(
    //         previous_impedance: Complex32,
    //         circuit: &str,
    //         mut depth: i32,
    //         params_complex: &HashMap<&str, &[Complex32]>,
    //         params_f32: &HashMap<&str, &[f32]>,
    //         frequency: f32,
    //         element_counter: &HashMap<&str, usize>,
    //     ) -> Complex32 {
    //         let compared_char = circuit.chars().next();

    //         if compared_char == Some(')') && (depth == 1 || depth == 0) {
    //             depth -= 1;
    //             return Complex32 { re: 1., im: 0. }
    //                 / (inner_impedance(
    //                     previous_impedance,
    //                     &circuit[1..],
    //                     depth,
    //                     params_complex,
    //                     params_f32,
    //                     frequency,
    //                     element_counter,
    //                 ) + previous_impedance);
    //         } else if compared_char == Some('(') {
    //             depth += 1;
    //             return previous_impedance
    //                 + inner_impedance(
    //                     previous_impedance,
    //                     &circuit[1..],
    //                     depth,
    //                     params_complex,
    //                     params_f32,
    //                     frequency,
    //                     element_counter,
    //                 );
    //         } else if compared_char == None {
    //             return previous_impedance;
    //         } else {
    //             return previous_impedance
    //                 + inner_impedance(
    //                     previous_impedance,
    //                     &circuit[1..],
    //                     depth,
    //                     params_complex,
    //                     params_f32,
    //                     frequency,
    //                     element_counter,
    //                 )
    //                 + Complex32 { re: 1., im: 0. }
    //                     / Element::from_str(
    //                         &circuit[..1],
    //                         params_complex
    //                             .get(
    //                                 format!(
    //                                     "{}{}",
    //                                     &circuit[..1],
    //                                     element_counter
    //                                         .get(&circuit[..1])
    //                                         .expect("element_counter should have this key")
    //                                 )
    //                                 .as_str(),
    //                             )
    //                             .expect("params_complex should contain requested key"),
    //                         params_f32
    //                             .get(
    //                                 format!(
    //                                     "{}{}",
    //                                     &circuit[..1],
    //                                     element_counter
    //                                         .get(&circuit[..1])
    //                                         .expect("element_counter should have this key")
    //                                 )
    //                                 .as_str(),
    //                             )
    //                             .expect("params_f32 should contain requested key"),
    //                     )
    //                     .impedance(frequency);
    //         }
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let circuit_schema = String::from("RCRC");
        let circuit_model = CircuitModel::new(circuit_schema.clone());
        let mut params_complex = HashMap::new();
        params_complex.insert("R0", [Complex32 { re: 1., im: 1. }].as_slice());
        params_complex.insert("C0", [Complex32 { re: 3., im: 3. }].as_slice());
        params_complex.insert("R1", [Complex32 { re: 2., im: 2. }].as_slice());
        params_complex.insert("C1", [Complex32 { re: 4., im: 4. }].as_slice());

        let mut params_f32 = HashMap::new();
        params_f32.insert("R0", [1.].as_slice());
        params_f32.insert("C0", [1.].as_slice());
        params_f32.insert("R1", [1.].as_slice());
        params_f32.insert("C1", [1.].as_slice());

        assert_eq!(
            circuit_model.impedance(&params_complex, &params_f32, 1.),
            Complex32 {
                re: 2.9469484,
                im: 2.9469484
            }
        )
    }

    #[test]
    fn case_1() {
        let circuit_schema = String::from("RC(RC)");
        let circuit_model = CircuitModel::new(circuit_schema.clone());
        let mut params_complex = HashMap::new();
        params_complex.insert("R0", [Complex32 { re: 1., im: 1. }].as_slice());
        params_complex.insert("C0", [Complex32 { re: 3., im: 3. }].as_slice());
        params_complex.insert("R1", [Complex32 { re: 2., im: 2. }].as_slice());
        params_complex.insert("C1", [Complex32 { re: 4., im: 4. }].as_slice());

        let mut params_f32 = HashMap::new();
        params_f32.insert("R0", [1.].as_slice());
        params_f32.insert("C0", [1.].as_slice());
        params_f32.insert("R1", [1.].as_slice());
        params_f32.insert("C1", [1.].as_slice());

        // Circuit: R0 + C0 + 1/(1/R1 + 1/C1)
        // With the given complex values, this should compute to a specific impedance
        assert_eq!(
            circuit_model.impedance(&params_complex, &params_f32, 1.),
            Complex32 { re: 1., im: 1. }
        )
    }
}
