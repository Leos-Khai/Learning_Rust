use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    println!("Secret number: {}", secret_number);

    loop {
        println!("Enter a number.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Fail");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your guess is: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
