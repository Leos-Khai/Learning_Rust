fn main() {
    let mut count = 0;
    'counting_down: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_down;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
    let mut thing = 5;
    while thing > 0 {
        println!("thing = {}", thing);
        thing -= 1;
    }
    let thingbox: [i32; 5] = [1, 2, 3, 4, 5];
    let mut tracker = 0;
    while tracker < thingbox.len() {
        println!("box has {}", thingbox[tracker]);
        tracker += 1;
    }
    for x in thingbox {
        println!("thing {}", x);
    }
    let mut stupid = 0;
    for x in (55..1).rev() {
        stupid += x;
        println!("Super stupid {}", stupid);
    }
}
