fn main() {
    // functions()
    // loop_loop()
    for_loop()
}

fn functions() {
    let y = {
        let x: i32 = 3;
        another_function(x);
        x + 1
    };
    println!("The value of y is: {:#?}", y);
}

fn another_function(x: i32) {
    println!("The value of x is: {:#?}", x);
}

fn loop_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn for_loop () {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
