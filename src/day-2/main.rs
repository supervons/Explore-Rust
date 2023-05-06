fn main() {
    // error
    let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1); // error
    let _s2 = s1.clone();
    println!("{}, world!", s1); // error

    // success
    let x: &str = "hello, world";
    let y = x;
    println!("{},{}", x, y);
}
