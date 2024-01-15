use ark_ff::fields::{Fp64, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "15328968290826396851"]
#[generator = "6"]
pub struct FrConfig;
pub type Fr = Fp64<MontBackend<FrConfig, 1>>;
