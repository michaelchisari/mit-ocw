// MIT 6.00SC
// Problem Set #002-a
// Polynomials

use std::f64;

// Evaluate polynomials
fn evaluate_poly(poly: &[f64], x: f64) -> f64 {

    let len = poly.len() - 1;

    let mut i = len as i8;

    let mut return_value = 0.0;

    while i >= 0 {
        let p = i as usize;

        return_value += poly[p] * x.powf(i as f64);

        i -= 1;
    }

    return return_value;
}

fn main() {

    // f(-13) = 7.0(-13)4 + 9.3(-13)3 + 5.0(-13)2
    let e = evaluate_poly(&[0.0, 0.0, 5.0, 9.3, 7.0], -13.0);

    println!("{}", e);
}
