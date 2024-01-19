use crate::fields;
use ark_ff::{Field, MontFp};
use ark_ec::{
    models::{short_weierstrass::SWCurveConfig, CurveConfig},
    short_weierstrass::{Affine, Projective},
};

/// G1_GENERATOR_X
pub const G1_GENERATOR_X: fields::Fq = MontFp!("3510221");

/// G1_GENERATOR_Y
pub const G1_GENERATOR_Y: fields::Fq = MontFp!("4798840");

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Config;

pub type GAffine = Affine<Config>;
pub type GProjective = Projective<Config>;

impl CurveConfig for Config {
    type BaseField = fields::Fq;
    type ScalarField = fields::Fr;

    /// COFACTOR = 1
    const COFACTOR: &'static [u64] = &[0x1];

    /// COFACTOR_INV = COFACTOR^{-1} mod r = 1
    const COFACTOR_INV: fields::Fr = fields::Fr::ONE;
}

impl SWCurveConfig for Config {
    /// COEFF_A
    const COEFF_A: fields::Fq = MontFp!("3558150");

    /// COEFF_B
    const COEFF_B: fields::Fq = MontFp!("8339963");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const GENERATOR: GAffine = GAffine::new_unchecked(G1_GENERATOR_X, G1_GENERATOR_Y);
}