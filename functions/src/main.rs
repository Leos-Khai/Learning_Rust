fn main() {
    println!("Main");
    not_main(3.3, "stupid");
    let x = plus_one(5);
    println!("X is: {}", x);
}
fn not_main(x: f32, y: &str) {
    println!("not main {} is {}", x, y);
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
