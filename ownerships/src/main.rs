fn main() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("{s}");

    let word = rewrite_first_word(&s);
    
    // s.clear();
    println!("the first word is: {}", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    println!("slice: {:#?}", slice);
    assert_eq!(slice, &[2, 3]);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// Using Slice
fn rewrite_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
