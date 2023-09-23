use std::{collections::HashMap, vec};
use std::time::Instant;


fn main() {
    overwriting_value();
    adding_a_key_and_value_only_if_a_key_is_not_present();
    updating_a_value_based_on_the_old_value();
        
    let mut v = vec![1, 2, 3, 4, 5];
    assert_eq!(find_median(&mut v), 3);
    assert_eq!(find_mode(&mut v).len(), 5);

    let mut v1 = vec![4, 2, 6, 3, 1, 5];
    assert_eq!(find_median(&mut v1), 3);
    assert_eq!(find_mode(&mut v1).len(), 6);

    let mut v2 = vec![5, 2, 6, 3, 1, 5];
    assert_eq!(find_mode(&mut v2).len(), 1);

    let before = Instant::now();
    assert_eq!(convert_string_to_pig_latin("first apple"), "irst-fay apple-hey");
    convert_string_to_pig_latin("first apple");
    println!("Elapsed time: {:.2?}", before.elapsed());

    let before1 = Instant::now();
    convert_string_to_pig_latin1("first apple");
    assert_eq!(convert_string_to_pig_latin1("first apple"), "irst-fay apple-hey");
    println!("Elapsed time: {:.2?}", before1.elapsed());
     
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

fn find_mode(list: &mut Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();

    for num in list {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut max = 0;

    let mut modes: Vec<i32> = Vec::new();

    for (_, &value) in map.iter() {
        if value > max {
            max = value;
        }
    }

    for (key, value) in map {
        if value == max {
            modes.push(*key);
        }
    }
    return modes;
}

fn convert_string_to_pig_latin(sentence: &str) -> String {
    let mut pig_latin = String::new();
    let words = sentence.split_whitespace();

    for word in words {
        let mut chars = word.chars();
        let first_char = chars.next().unwrap();
        let rest = chars.as_str();

        let mut new_word = String::new();
        if first_char == 'a' || first_char == 'e' || first_char == 'i' || first_char == 'o' || first_char == 'u' {
            new_word.push_str(&word);
            new_word.push_str("-hey");
            pig_latin.push_str(&new_word);
            pig_latin.push(' ');
            continue;
        }

        new_word.push_str(rest);
        new_word.push('-');
        new_word.push(first_char);
        new_word.push_str("ay");

        pig_latin.push_str(&new_word);
        pig_latin.push(' ');
    }
    return pig_latin.trim().to_string();
}

fn convert_string_to_pig_latin1(text: &str) -> String {
    let mut result: Vec<String> = Vec::new();
    for word in text.split_whitespace() {
        let first_char = &word.chars().next();
        if let Some(c) = &first_char {
            let mut w = String::from(word);
            if c == &'a' || c == &'e' || c == &'i' || c == &'o' || c == &'u' {
                result.push(w + "-hey");
            } else {
                let removed_c = w.remove(0);
                w.push('-');
                w.push(removed_c);
                w.push_str("ay");
                result.push(w)
            }
        }
    }
    result.join(" ")
}

fn is_vowels(c: &char) -> bool {
    let vowels: &str = "aeiou";
    for v in vowels.chars() {
        if v == *c {
            return true;
        }
    }
    return false;
}