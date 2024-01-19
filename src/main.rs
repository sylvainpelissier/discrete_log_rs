use ark_ec::Group;
use ark_std::{rand, UniformRand};
use ark_ff::{Field, MontFp, PrimeField};
use num_bigint::BigUint;

pub mod fields;
pub mod curves;

use crate::curves::{GAffine, GProjective};
use crate::fields::{Fr};


const ONE : Fr = MontFp!("1");
const TWO : Fr = MontFp!("2");

fn update(P : curves::GProjective, Q : curves::GProjective, a_i: &mut Fr, b_i: &mut Fr, X_i : &mut GProjective) {
    let case : BigUint = (X_i.x).into();
    let case: u64 = case.to_u64_digits()[0];
    match case % 3 {
        0 => {  
                *b_i = *b_i + ONE;
                *X_i = *X_i + Q;
        }
        1 => {  *a_i = *a_i * TWO;
                *b_i = *b_i * TWO;
                *X_i = X_i.double();
        }
        2 => {  *a_i = *a_i + ONE;
                *X_i = *X_i + P;
        }
        _ => panic!("Wrong case.")
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    // Inputs
    let i: fields::Fq = MontFp!("3510221");
    let P = GAffine::get_point_from_x_unchecked(i, false).unwrap();
    let P: GProjective = P.into(); 
    let x: Fr = Fr::rand(&mut rng);
    println!("x = {}", x);
    let Q = P * x;
    println!("P {:?}", P);
    println!("Q {:?}", Q);
    // Temporary variable
     
    let modulus: BigUint = (Fr::MODULUS).into();
    let sqrt = (modulus.to_u64_digits()[0] as f64).sqrt() as u64;
    println!("sqrt {:?}", sqrt);
    let mut xp = x + ONE;
    let mut i: u64 = 0;
    let mut j: u64 = 0;
    while j < 1000 && x != xp {
        let mut a_i: Fr = Fr::rand(&mut rng);
        let mut b_i: Fr = Fr::rand(&mut rng);
        let mut a_2i: Fr = Fr::rand(&mut rng);
        let mut b_2i: Fr = Fr::rand(&mut rng);
        
        let mut X_i = P*a_i + Q*b_i;
        let mut X_2i = P*a_2i + Q*b_2i;
        
        //println!("X_i {:?}", X_i);
        i = 0;
        while i <  100 * sqrt {
            update(P,  Q, &mut a_i, &mut b_i, &mut X_i);
            
            // Double step
            update(P,  Q, &mut a_2i, &mut b_2i, &mut X_2i);
            update(P,  Q, &mut a_2i, &mut b_2i, &mut X_2i);

            if X_i == X_2i {
                xp = (b_i - b_2i).inverse().unwrap() * (a_2i - a_i);
                break;
            }
            i +=1;
        }
        j+=1;
    }
    if j < 1000 {
        println!("j = {j}");
        println!("i = {i}");
        println!("x = {}", x);
        println!("Found a solution {}", xp);
        println!("Verification {}", P*xp);
    }
    else {
        println!("No solution found");
    }
}
