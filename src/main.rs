use ark_ff::MontFp;

pub mod fields;
pub mod curves;

fn main() {
    let x: fields::Fq = MontFp!("2");
    let a = curves::GAffine::get_point_from_x_unchecked(x, true).unwrap();
    println!("Test {:?}", a);
}
