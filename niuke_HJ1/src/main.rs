use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    // let mut words = input.split_whitespace().collect::<Vec<&str>>();
    let it = input.split_whitespace();
    let last_word = it.last().unwrap();

    let mut out = String::new();
    out.push_str(&last_word.len().to_string());
    out.push('\n');
    print!("{out}");
}
