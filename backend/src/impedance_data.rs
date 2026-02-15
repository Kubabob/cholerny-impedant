use std::{error::Error, fs::File, path::Path};

use csv::Reader;
use num::complex::Complex32;

pub struct ImpedanceData {
    pub frequencies: Vec<f32>,
    pub z_exp: Vec<Complex32>,
    pub z_pred: Option<Vec<Complex32>>,
}

impl ImpedanceData {
    fn new(frequencies: Vec<f32>, z_exp: Vec<Complex32>, z_pred: Option<Vec<Complex32>>) -> Self {
        Self {
            frequencies: frequencies,
            z_exp: z_exp,
            z_pred: z_pred,
        }
    }

    fn validate(self) {
        assert_eq!(self.frequencies.len(), self.z_exp.len());

        if let Some(z_pred) = self.z_pred {
            assert_eq!(self.frequencies.len(), z_pred.len());
        }
    }

    fn residue(self) -> Vec<Complex32> {
        self.z_exp
            .iter()
            .zip(self.z_pred.expect("Z_pred does not exist"))
            .map(|(val_a, val_b)| val_a - val_b)
            .collect()
    }

    fn chi2(self) -> f32 {
        self.residue()
            .iter()
            .fold(0.0, |acc, val| acc + (val.im.powi(2) + val.re.powi(2)))
    }

    fn read_csv(self, file_path: String) -> Result<Self, Box<dyn Error>> {
        let file_path = Path::new(&file_path);

        assert!(
            &["txt", "csv"].contains(
                &file_path
                    .extension()
                    .expect("{file_path:?} does not have extension")
                    .to_ascii_lowercase()
                    .to_str()
                    .expect("Could not convert OsStr to str")
            )
        );

        let file = File::open(file_path).expect("Could not open file {file_path:?}");
        let mut reader = Reader::from_reader(file);

        let header = reader
            .headers()
            .expect("Could not retrieve the header from file");

        assert_eq!(
            header,
            vec!["f", "Z'", "-Z''"],
            "File contains wrong header: {header:#?}"
        );

        let mut frequencies: Vec<f32> = Vec::new();
        let mut z_exp: Vec<Complex32> = Vec::new();

        for record in reader.records() {
            let line = record.expect("Record is not accesible");

            frequencies.push(line[0].parse().expect("Could not parse frequency to f32"));
            let z_real: f32 = line[1].parse().expect("Could not parse Z' to f32");
            let z_imag: f32 = line[2].parse().expect("Could not parse -Z'' to f32");
            z_exp.push(Complex32 {
                re: z_real,
                im: z_imag,
            });
        }

        Ok(Self {
            frequencies: frequencies,
            z_exp: z_exp,
            z_pred: None,
        })
    }
}

impl Default for ImpedanceData {
    fn default() -> Self {
        Self::new(Vec::new(), Vec::new(), None)
    }
}
