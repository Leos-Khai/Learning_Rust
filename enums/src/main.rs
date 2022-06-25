enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter,
}

fn value_sents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 2,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("hi");
    if let Some(num) = Coin::Penny {
        thing
    }
}
