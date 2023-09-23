use std::io;

fn main() {
    println!("Welcome to Generate the nth Fibonacci number program");

    loop {
        println!("Please input the nth.");
        let mut nth = String::new();

        io::stdin()
            .read_line(&mut nth)
            .expect("Failed to read line. read temperature value");
        
        let nth: usize = match nth.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Output: {}", fib_dynamic(nth));
    
    }
}

fn fib_dynamic (n: usize) -> usize {
    let mut f = vec![0; n + 2];
    let mut i = 2;

    f[0] = 0;
    f[1] = 1;
    while i < f.len() {

        f[i] = f[i-1] + f[i-2];

        i += 1;
    }
    return f[n];
}

fn fib_recursive (n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    {
        return fib_recursive(n - 1) + fib_recursive(n - 2);
    }
}

