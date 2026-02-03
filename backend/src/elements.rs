use std::f32::consts::PI;

use num::complex::Complex32;

#[allow(non_snake_case)]
pub enum Element {
    R {
        R: Complex32,
    }, // Resistor
    C {
        C: Complex32,
    }, // Capacitor
    L {
        L: Complex32,
    }, // Inductor
    W {
        Aw: Complex32,
    }, // Semi-infinite Warburg element
    Wo {
        Z0: Complex32,
        tau: f32,
    }, // Open (finite-space) Warburg element
    Ws {
        Z0: Complex32,
        tau: f32,
    }, // Short (finite-length) Warburg element
    CPE {
        Q: Complex32,
        alpha: f32,
    }, // Constant phase element
    La {
        L: Complex32,
        alpha: f32,
    }, // Modified inductance element
    G {
        R_G: Complex32,
        t_G: f32,
    }, // Gerischer element
    Gs {
        R_G: Complex32,
        t_G: f32,
        phi: f32,
    }, // Finite-length Gerischer element
    K {
        R: Complex32,
        tau_k: f32,
    }, // RC element for use in lin-KK model
    Zarc {
        R: Complex32,
        tau_k: f32,
        gamma: f32,
    }, // RQ element rewritten with resistance and time constant as parameters. Equivalent to a Cole-Cole relaxation in dielectrics.
    TLMQ {
        Rion: Complex32,
        Qs: f32,
        gamma: f32,
    }, // Simplified transmission-line model
    T {
        A: Complex32,
        B: Complex32,
        a: f32,
        b: f32,
    }, // Macrohomogeneous porous electrode model from Paasch et al.
}

impl Element {
    pub fn impedance(&self, frequency: f32) -> Complex32 {
        let omega = 2. * PI * frequency;
        let i = Complex32::new(0.0, 1.0);

        match self {
            Element::R { R } => *R,
            Element::C { C } => 1.0 / (C * i * omega),
            Element::L { L } => L * i * omega,
            Element::W { Aw } => Aw * Complex32::new(1.0, -1.0) / omega.sqrt(),
            Element::Wo { Z0, tau } => {
                let s = (i * omega * tau).sqrt();
                Z0 / (s * s.tanh())
            }
            Element::Ws { Z0, tau } => {
                let s = (i * omega * tau).sqrt();
                Z0 * s.tanh() / s
            }
            Element::CPE { Q, alpha } => 1.0 / (Q * (i * omega).powf(*alpha)),
            Element::La { L, alpha } => (L * i * omega).powf(*alpha),
            Element::G { R_G, t_G } => R_G / (1.0 + i * omega * t_G).sqrt(),
            Element::Gs { R_G, t_G, phi } => {
                let s = (1.0 + i * omega * t_G).sqrt();
                R_G / (s * (*phi * s).tanh())
            }
            Element::K { R, tau_k } => R / (1.0 + i * omega * tau_k),
            Element::Zarc { R, tau_k, gamma } => R / (1.0 + (i * omega * tau_k).powf(*gamma)),
            Element::TLMQ { Rion, Qs, gamma } => {
                let zs = 1.0 / (Qs * (i * omega).powf(*gamma));
                (Rion * zs).sqrt() / (Rion / zs).sqrt().tanh()
            }
            Element::T { A, B, a, b } => {
                let beta = Complex32::new(*a, omega * b).sqrt();
                let sinh_beta = if beta.re < 100.0 {
                    beta.sinh()
                } else {
                    // Prevent overflow
                    Complex32::new(1e10, 0.0)
                };
                A / (beta * beta.tanh()) + B / (beta * sinh_beta)
            }
        }
    }

    pub fn from_str(element_str: &str, params_complex: &[Complex32], params_f32: &[f32]) -> Self {
        match element_str {
            "R" => Self::R {
                R: params_complex[0],
            },
            "C" => Self::C {
                C: params_complex[0],
            },
            "L" => Self::L {
                L: params_complex[0],
            },
            "W" => Self::W {
                Aw: params_complex[0],
            },
            "Wo" => Self::Wo {
                Z0: params_complex[0],
                tau: params_f32[0],
            },
            "Ws" => Self::Ws {
                Z0: params_complex[0],
                tau: params_f32[0],
            },
            "CPE" => Self::CPE {
                Q: params_complex[0],
                alpha: params_f32[0],
            },
            "La" => Self::La {
                L: params_complex[0],
                alpha: params_f32[0],
            },
            "G" => Self::G {
                R_G: params_complex[0],
                t_G: params_f32[0],
            },
            "Gs" => Self::Gs {
                R_G: params_complex[0],
                t_G: params_f32[0],
                phi: params_f32[1],
            },
            "K" => Self::K {
                R: params_complex[0],
                tau_k: params_f32[0],
            },
            "Zarc" => Self::Zarc {
                R: params_complex[0],
                tau_k: params_f32[0],
                gamma: params_f32[1],
            },
            "TLMQ" => Self::TLMQ {
                Rion: params_complex[0],
                Qs: params_f32[0],
                gamma: params_f32[1],
            },
            "T" => Self::T {
                A: params_complex[0],
                B: params_complex[1],
                a: params_f32[0],
                b: params_f32[1],
            },
            _ => panic!("Unknown element type: {}", element_str),
        }
    }
}
