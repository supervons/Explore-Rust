use num::complex::Complex;
fn greet_world() {
    let result = add(18, 19);
    println!("{}", result);
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn main() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);
    greet_world();
}
