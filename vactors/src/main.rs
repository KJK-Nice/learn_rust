fn main() {
    test3()
}


fn test4() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let first = &row[0];
}

fn test3() {
    let mut v = vec![100, 30, 50];
    for i in &v {
        println!("{}", i);
    }
    for i in &mut v {
        *i += 1;
    }
    for i in &v {
        println!("{}", i);
    }
}

fn test2() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // v.push(6);

    println!("The first element is: {}", first);
}

fn test() {
    let mut v: Vec<i32> = Vec::new();
    
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(5);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}