use std::io;

fn main() {
    println!("Welcome to Convert Temperatures program");
    println!("Please select convert mode.\n1. f2c\n2. c2f");

    let mut convert_mode = String::new();

    io::stdin()
        .read_line(&mut convert_mode)
        .expect("Failed to read line. read convert mode");
    
    let convert_mode = convert_mode.trim();

    loop {
        println!("Please input the value.");
        let mut temperature_value = String::new();

        io::stdin()
            .read_line(&mut temperature_value)
            .expect("Failed to read line. read temperature value");
        
        let temperature_value: f64 = match temperature_value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if convert_mode == "f2c" {
            let c = (temperature_value - 32.0) * 5.0 / 9.0;
            println!("👉 {} C", c);
        } else if convert_mode == "c2f" {
            let f = (temperature_value * 9.0 / 5.0) + 32.0;
            println!("👉 {} F", f);
        }

    }
}

