use num::complex::Complex32;

pub struct ImpedanceData {
    f: Vec<f32>,
    z_exp: Vec<Complex32>,
    z_pred: Vec<Complex32>,
}

impl ImpedanceData {
    fn new() -> Self {
        todo!()
    }

    fn validate(self) {
        if self.f.len() != self.z_exp.len() && self.f.len() != self.z_pred.len() {
            panic!("Data vectors are of different size")
        }
    }

    fn residue(self) -> Vec<Complex32> {
        self.z_exp
            .iter()
            .zip(self.z_pred)
            .map(|(val_a, val_b)| *val_a - val_b)
            .collect()
    }

    fn chi2(self) -> f32 {
        self.residue()
            .iter()
            .fold(0.0, |acc, val| acc + (val.im.powi(2) + val.re.powi(2)))
    }
}
