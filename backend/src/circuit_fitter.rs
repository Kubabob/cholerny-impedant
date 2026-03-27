use argmin::core::CostFunction;
use num::traits::Float;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use std::sync::{Arc, Mutex};

use crate::{circuit_model::CircuitModel, impedance_data::ImpedanceData};

struct CircuitFitter<'a> {
    circuit_model: CircuitModel<'a>,
    impedance_data: ImpedanceData,
    lower_bound: Vec<f32>,
    upper_bound: Vec<f32>,
    rng: Arc<Mutex<Xoshiro256PlusPlus>>,
}

impl<'a> CircuitFitter<'a> {
    pub fn new(
        circuit_model: CircuitModel<'a>,
        impedance_data: ImpedanceData,
        lower_bound: Vec<f32>,
        upper_bound: Vec<f32>,
    ) -> Self {
        Self {
            circuit_model,
            impedance_data,
            lower_bound,
            upper_bound,
            rng: Arc::new(Mutex::new(Xoshiro256PlusPlus::from_seed([42; 32]))),
        }
    }
}

impl<'a> CostFunction for CircuitFitter<'a> {
    type Param = Vec<f32>;
    type Output = f32;

    fn cost(&self, param: &Self::Param) -> Result<Self::Output, argmin_math::Error> {
        self.impedance_data
            .frequencies
            .iter()
            .map(|freq| self.circuit_model.impedance(freq));
        Ok(0.)
    }
}
