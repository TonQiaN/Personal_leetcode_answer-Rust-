use std::io::{self, Read};

fn main() {
    let (mut input, mut output) = (String::new(), String::new());
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let line_vec = line
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let sum = line_vec.iter().sum::<i32>();
        output.push_str(&sum.to_string());
        output.push('\n');
    }
    print!("{}", output);
}
