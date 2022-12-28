extern crate proc_macro2;
use std::fs;
use quote::quote;
struct Monkey{
    items: Vec<i32>,
    test: i32,
    operation: fn(i32) -> i32,
}


fn main() {
    let mut lines: Vec<&str> = fs::read_to_string("src/input").unwrap().split("\n\n").collect();
    let mut monkeys: Vec<Monkey> = Vec::new();
    for x in lines{
        let tokens: proc_macro2::TokenStream = proc_macro2::parse("let x = 1");
    }
}
