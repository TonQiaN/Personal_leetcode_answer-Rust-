use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut my_set = vec![false; 128];
    let s = it.next().unwrap().as_bytes();
    for &c in s {
        let c = c as usize;
        my_set[c] = true;
    }
    let result = my_set.iter().filter(|&&x| x).count();
    println!("{}", result);
}