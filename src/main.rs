fn greet_world() {
    let result = add(18, 19);
    println!("{}", result);
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn main() {
    let c = 'z';
    let heart_eyed_cat = '😻';
    println!("{} + {}", c, heart_eyed_cat);
    greet_world();
}
