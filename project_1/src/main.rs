use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Project 1:");
    println!("1. For fibonacci:");
    println!("2. Temprature conversion");
    println!("3. Christmas song:");
    println!("4. Guess the number game.:");
    println!("5. Exit:");
    loop {
        println!("Choose (1, 2, 3, 4, 5):");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("not number");
        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match choice {
            1 => run_fibo(),
            2 => run_tempo(),
            3 => christmas_song(),
            4 => guess_the_number(),
            5 => break,
            _ => continue,
        };
    }
}
fn run_fibo() {
    println!("Choose a nth number you want to get the fibonacci number of:");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Not a number");
    let num: u32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };
    fibo(num);
    println!("{}th term is {}", num, fib(num));
}

fn fibo(n: u32) {
    if n <= 2 {
        println!("{}", n);
    } else {
        let mut f0: u128 = 0;
        let mut f1: u128 = 1;
        let mut fnth: u128;
        println!("0. {}", f0);
        println!("1. {}", f1);
        for i in 2..n+1 {
            fnth = f0 + f1;
            println!("{}. {}", i, fnth);
            f0 = f1;
            f1 = fnth;
        }
    }
}

fn fib(x: u32) -> u32 {
    if x < 2 {
        return x;
    }
    return fib(x - 1) + fib(x - 2);
}

fn run_tempo() {
    println!("Enter a number to represent a tempreture:");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("not number");
    println!("Enter (C) to convert to Ferenhight and (F) to convert it to celcious:");
    let mut temp_s = String::new();
    io::stdin().read_line(&mut temp_s).expect("Not");
    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => 1.5,
    };
    let temp_s: char = match temp_s.trim().parse() {
        Ok(num) => num,
        Err(_) => 'f',
    };
    convert_tempo(temp, temp_s);
}

fn convert_tempo(x: f64, y: char) {
    if y == 'c' {
        println!("{}C is {}F", x, x * (9.0 / 5.0) + 30.0);
    } else if y == 'f' {
        println!("{}F is {}C", x, (x - 32.0) * 5.0 / 9.0);
    }
}

fn christmas_song() {
    for i in (1..13).rev() {
        match i {
            12 => println!("{} something.", i),
            11 => println!("{} something.", i),
            10 => println!("{} something.", i),
            9 => println!("{} something.", i),
            8 => println!("{} something.", i),
            7 => println!("{} something.", i),
            6 => println!("{} something.", i),
            5 => println!("{} something.", i),
            4 => println!("{} something.", i),
            3 => println!("{} something.", i),
            2 => println!("{} something.", i),
            1 => println!("{} something.", i),
            _ => println!("Er wrong song."),
        }
    }
}

fn guess_the_number() {
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
