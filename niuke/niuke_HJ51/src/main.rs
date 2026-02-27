use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.lines();
    while let (Some(n), Some(line), Some(k)) = (it.next(), it.next(), it.next()) {
        let mut line_vec = line
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        line_vec.reverse();
        let order = k.trim().parse::<usize>().unwrap();
        println!("{}", line_vec[order - 1]);
    }
}
