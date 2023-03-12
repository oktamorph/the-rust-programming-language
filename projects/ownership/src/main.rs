use std::{mem::take, string};

fn main() {
    // let s = "hello";

    // let mut s = String::from("hello");
    // s.push_str(", world!");
    // println!("{}", s);

    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1);

    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1 = {}, s2 = {}", s1, s2);

    // let s = String::from("hello");
    // takes_ownership(s);

    // let x = 5;
    // makes_copy(x);

    // println!("s: {}", s);
    // println!("x: {}", x);

    // let s1 = gives_ownership();

    // let s2 = String::from("hello");

    // let s3 = takes_and_gives_back(s2);

    // let(s2, len) = calculate_length(s1);

    // println!("The length of '{}' is {}.", s2, len);

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    change(&s1);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();

//     (s, length)
// }

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}
