use crate::fields;
use ark_ff::{Field, MontFp};
use ark_ec::{
    models::{short_weierstrass::SWCurveConfig, CurveConfig},
    short_weierstrass::{Affine},
};

/// G1_GENERATOR_X
pub const G1_GENERATOR_X: fields::Fq = MontFp!("11237173340785249450");

/// G1_GENERATOR_Y
pub const G1_GENERATOR_Y: fields::Fq = MontFp!("10893031506520704136");

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Config;

pub type GAffine = Affine<Config>;

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
    const COEFF_A: fields::Fq = MontFp!("6211002006250565559");

    /// COEFF_B
    const COEFF_B: fields::Fq = MontFp!("14749063037593397239");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const GENERATOR: GAffine = GAffine::new_unchecked(G1_GENERATOR_X, G1_GENERATOR_Y);
}