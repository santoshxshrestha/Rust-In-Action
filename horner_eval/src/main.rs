#![allow(unused)]

fn horners_eval(coeffs: &[f64], x: f64) -> f64 {
    let mut result = coeffs[0];

    for &coeff in &coeffs[1..] {
        // the formula is  bn = an
        // bn-1 = an-1 + bnX0
        // last to times x + secend last ko I mean from the sense of the greater degree to the lower degree
        result = result * x + coeff;
    }
    result
}

fn main() {
    // f(x) = x^4 - 2x^3 - 13x^2 + 38x - 24
    let coeffs = [1.0, -2.0, -13.0, 38.0, -24.0];
    let x = 2.0;
    let result = horners_eval(&coeffs, x);
    println!("f({}) = {}", x, result); // should print f(2) = 0
}
