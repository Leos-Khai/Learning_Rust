fn main() {
    let something = true;
    let number = if something { 5 } else { 3 };
    if number < 3 {
        println!("Greater");
    } else if number == 5 {
        println!("Nice");
    } else {
        println!("Something else");
    }
}
