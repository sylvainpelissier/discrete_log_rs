use ark_ff::fields::{Fp64, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "15328968294814049009"]
#[generator = "6"]
pub struct FqConfig;
pub type Fq = Fp64<MontBackend<FqConfig, 1>>;
