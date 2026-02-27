use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    it.next();
    let mut result = it
        .map(|str| str.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    result.sort_unstable();
    result.dedup();
    let mut out = String::new();
    for i in result {
        out.push_str(&i.to_string());
        out.push('\n');
    }
    print!("{}", out);
}
