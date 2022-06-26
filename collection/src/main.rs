fn main() {
    run_vec();
    run_enum_vec();
}

fn run_vec() {
    println!("Hello, world!");
    // let v: Vec<i32> = Vec::new();
    let mut v: Vec<i32> = vec![1, 2, 3, 55, 54, 53];
    for i in &v {
        //let index: usize = *i - 1;
        //let thing: &usize = &v[index];
        //let thing = v.get(index);
        println!("{:?}, {}", i, i);
    }
    print_vec(&mut v);
    for i in &v {
        //let index: usize = *i - 1;
        //let thing: &usize = &v[index];
        //let thing = v.get(index);
        println!("{:?}, {}", i, i);
    }
}

fn print_vec(vec: &mut Vec<i32>) {
    for i in vec {
        *i += 2;
    }
}

#[derive(Debug)]
enum Cell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn run_enum_vec() {
    let row = vec![
        Cell::Int(5),
        Cell::Float(2.2),
        Cell::Text(String::from("HiWo")),
    ];
    for i in &row {
        println!("{:?}", i);
    }
}
