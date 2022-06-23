fn main() {
    let mut s = String::from("Something");
    let s2 = s.clone();
    s.push_str(", Thing");
    let mut x = 5;
    let y = x;
    x = 10;
    s = plus(s);
    let (mut s, z) = super_function(s);
    only_ref(&mut s);
    println!("{} {} {} {} {}", s, x, y, s2, z);

    let mut w = String::from("Mew");
    {
        let f = &mut w;
        f.push_str("two");
    }
    let g = &mut w;
    g.push_str(" is strong.");
    println!("{}", w);
    let h = String::from("This is a string  我");
    let jerk = get_first_word(&h);
    let mut notjerk = "Hello World";
    println!("{}", notjerk);
    string_breaker(notjerk);
    notjerk = "jerk";
    println!("{}, {}, {}", jerk, h, notjerk);
}

fn get_first_word(s: &String) -> &str {
    let b = s.as_bytes();
    for (j, &item) in b.iter().enumerate() {
        println!("{}, {}", item, j);
    }
    for (i, &item) in b.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn plus(mut num: String) -> String {
    println!("{}", num);
    num.push_str(", More, Thing");
    return num;
}

fn super_function(num: String) -> (String, i32) {
    return (num, 50);
}

fn only_ref(s: &mut String) {
    s.push_str(". So much");
}

fn string_breaker(s: &str) {
    println!("String {}", s);
}
