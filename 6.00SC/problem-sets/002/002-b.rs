// MIT 6.00SC
// Problem Set #002-b
// Derivatives

use std::f64;

// Compute derivatives of a polynomial
fn compute_deriv(poly: Vec<f64>) -> Vec<f64> {

    let mut result: Vec<f64> = vec![];

    let mut i = 1.0;

    // Need to reverse!
    for p in poly {
        if p == 0.0 {
            result.push(0.0);
        } else {
            result.push(p * i);
            println!("{} {}", p, i);
        }

        i += 1.0;
    }

    return result;

}

fn main() {

    // f(x) = x^4 + 3.0x^3 + 17.5x^2 - 13.39
    let poly = vec![-13.39, 0.0, 17.5, 3.0, 1.0];
    let e = compute_deriv(poly);

    println!("{:?}", e);
}
