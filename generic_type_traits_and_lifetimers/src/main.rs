fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}


// Multiple Generic Data Types
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest char is {}", largest(&char_list));

    let both_integer = Point { x: 5, y: 10};
    let both_float = Point { x: 1.0, y: 4.0};
    let integer_and_float = Point { x: 5, y: 4.0};
    println!("both_integer.x = {}", both_integer.x);
    println!("both_float.y = {}", both_float.y);
    println!("integer_and_float.x = {}", integer_and_float.x);
}

