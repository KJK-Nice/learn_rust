use std::collections::HashMap;

fn main() {
    overwriting_value();
    adding_a_key_and_value_only_if_a_key_is_not_present();
    updating_a_value_based_on_the_old_value();
        
    let mut v = vec![1, 2, 3, 4, 5];
    assert_eq!(find_median(&mut v), 3);

    let mut v1 = vec![4, 2, 6, 3, 1, 5];
    assert_eq!(find_median(&mut v1), 3);

    println!("v: {:?}", v);
    println!("v1: {:?}", v1);
}

// Example of how to use Hash Map
fn overwriting_value() {
    let mut scores = HashMap::new();
    let key = String::from("Blue");

    scores.insert(&key, 10);
    scores.insert(&key, 25);

    println!("{:?}", scores);
}

fn adding_a_key_and_value_only_if_a_key_is_not_present() {
    let mut scores = HashMap::new();
    let blue_key = String::from("Blue");
    let yello_key = String::from("Yellow");
    scores.insert(&blue_key, 10);

    scores.entry(&yello_key).or_insert(50);
    scores.entry(&blue_key).or_insert(50);

    println!("{:?}", scores);
}

fn updating_a_value_based_on_the_old_value() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

// Exercises
// Ex.1
fn find_median(list: &mut Vec<i32>) -> i32 {
    list.sort();
    let len: usize = list.len();
    let is_even: bool = len % 2 == 0;

    if is_even {
        let mid_right_index = &len / 2;
        let mid_left_index = &len / 2 - 1;
        let mid_right_value = list[mid_right_index];
        let mid_left_value = list[mid_left_index];
        return (mid_right_value + mid_left_value) / 2;
    } else {
        let mid_index = &len / 2;
        return list[mid_index];
    }
}


