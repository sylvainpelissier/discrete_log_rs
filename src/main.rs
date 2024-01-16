use ark_ec::Group;
use ark_ff::MontFp;
use num_bigint::BigUint;

pub mod fields;
pub mod curves;

use crate::curves::{GAffine, GProjective};
use crate::fields::{Fr, Fq};


fn update(P : curves::GProjective, Q : curves::GProjective, a_i: &mut Fr, b_i: &mut Fr, X_i : &mut GProjective) {
    let one : Fr = MontFp!("1");
    let two : Fr = MontFp!("2");
    let case : BigUint = (X_i.x).into();
    let case: u64 = case.to_u64_digits()[0];
    match case % 3 {
        0 => {  
                *b_i = *b_i + one;
                *X_i = *X_i + Q;
        }
        1 => {  *a_i = *a_i * two;
                *b_i = *b_i * two;
                *X_i = X_i.double();
        }
        2 => {  *a_i = *a_i + one;
                *X_i = *X_i + P;
        }
        _ => panic!("Wrong case.")
    }
}

fn main() {
    // Inputs
    let i: fields::Fq = MontFp!("2");
    let P = GAffine::get_point_from_x_unchecked(i, true).unwrap();
    let P: GProjective = P.into(); 
    let x: Fr = MontFp!("1337");
    let Q = P * x;

    // Temporary variable
    let i: Fq = MontFp!("4");
    let mut a_i: Fr = MontFp!("31");
    let mut b_i: Fr = MontFp!("78");
    let mut a_2i: Fr = MontFp!("1378");
    let mut b_2i: Fr = MontFp!("2379");
    
    let mut X_i = P*a_i + Q*b_i;
    let mut X_2i = P*a_2i + Q*b_2i;
    
    println!("X_i {:?}", X_i);
    println!("P {:?}", P);
    println!("Q {:?}", Q);

    let mut i: u64 = 0;
    while i < 5 {
        update(P,  Q, &mut a_i, &mut b_i, &mut X_i);
        
        // Double step
        update(P,  Q, &mut a_2i, &mut b_2i, &mut X_2i);
        update(P,  Q, &mut a_2i, &mut b_2i, &mut X_2i);
        /*println!("X_i {:?}", X_i);
        println!("X_2i {:?}", X_2i);
        println!("a_i {:?}", a_i);
        println!("b_i {:?}", b_i);
        println!("a_2i {:?}", a_2i);
        println!("b_2i {:?}", b_2i);*/

        if X_i == X_2i {
            break;
        }
        i +=1;

        if i % 1000 == 0{
            println!("i {:?}", i);
        }
    }
    println!("Found a solution {:?}", b_i);
}
