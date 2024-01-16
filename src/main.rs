use ark_ec::Group;
use ark_ff::MontFp;
use num_bigint::BigUint;

use crate::curves::GProjective;

pub mod fields;
pub mod curves;

fn f(P : curves::GProjective, Q : curves::GProjective, mut X_i : &mut curves::GProjective) {
    /*match P.x().rem(3).unrwap() {
        0 => 
    }*/
    let case : BigUint = (P.x).into();
    let case: u64 = case.to_u64_digits()[0];
    match case % 3 {
        0 => *X_i = *X_i + Q,
        1 => *X_i = X_i.double(),
        _ => *X_i = *X_i + P,
    }
}

fn main() {
    let x: fields::Fq = MontFp!("2");
    let a = curves::GAffine::get_point_from_x_unchecked(x, true).unwrap();
    println!("Test {:?}", a);
    let mut a : GProjective = a.into();
    f(a,  a, &mut a);
    println!("Test {:?}", a);
}
