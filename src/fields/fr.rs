use ark_ff::fields::{Fp64, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "9163699"]
#[generator = "2"]
pub struct FrConfig;
pub type Fr = Fp64<MontBackend<FrConfig, 1>>;
