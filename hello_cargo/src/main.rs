use hello_cargo::{hello::{self, helloworld}, plant::seed};

fn main() {
    println!("Hello, world!");
    println!("Basketball, Cat");
    //garden::goodbyeworld();
    //garden::helloworld();
    helloworld();
    hello::goodbye();
    seed();
}
