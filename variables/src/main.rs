// String, IU(8, 32, 64, 128, size), F(64, 32)
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
fn main() {
    let mut x = 5;
    println!("X is: {}", x);
    x = 6;
    println!("X is: {}", x);
    println!("3 hours in seconds is: {}", THREE_HOURS_IN_SECONDS);
    let y = 5;
    println!("{}", y);
    let y = x + 5;
    println!("{}", y);
    let y = y * 2;
    println!("{}", y);

    let z = "       ";
    let z = z.len();
    println!("{}", z);
}
