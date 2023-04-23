fn greet_world() {
    let southern_germany: &str = "Grüß Gott!";
    let chinese: &str = "世界，你好！";
    let english: &str = "World，hello";
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", region);
    }
}

fn main() {
    greet_world();
}
