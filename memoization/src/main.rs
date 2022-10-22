// https://codereview.stackexchange.com/questions/204555/recursive-fibonacci-in-rust-with-memoization
// https://users.rust-lang.org/t/fibonacci-with-memoization-how-to-store-hashmap/55989/3
use std::collections::HashMap;

fn main() {
    let mut cache: HashMap<usize, usize> = HashMap::default();
    println!("{}", fibo(100, &mut cache));
    println!("{}", fibo2(4));
}

fn fibo(n: usize, cache: &mut HashMap<usize, usize>) -> usize {
    if n <= 2 {
        return n;
    } else if let Some(&result) = cache.get(&n) {
        return result;
    } else {
        let result = fibo(n - 1, cache) + fibo(n - 2, cache);
        cache.insert(n, result);
        result
    }
}

fn fibo2(n: usize) -> usize {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    let mut n = n;
    while n > 0 {
        c = a + b;
        b = c;
        a = b;
        n = n - 1;
    }
    return c;
}
