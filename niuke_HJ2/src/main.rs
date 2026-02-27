use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    // let it = input.split_whitespace().collect::<Vec<&str>>();
    // let (word, target) = (it[0].as_bytes(), it[1].as_bytes());
    let mut lines = input.lines();
    let word = lines.next().unwrap().as_bytes().to_ascii_lowercase();
    let target = lines.next().unwrap().as_bytes().to_ascii_lowercase();
    let ans = word.into_iter().filter(|&c| c == target[0]).count();
    let mut out = String::new();
    out.push_str(&ans.to_string());
    out.push('\n');
    print!("{}", out)
}
