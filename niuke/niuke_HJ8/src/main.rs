use std::io::{self, *};

fn main() {
    use std::collections::BTreeMap;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines = input.lines();
    let mut record = BTreeMap::new();
    for line in lines.skip(1) {
        let mut line = line.split_whitespace();
        let (i, v) = (
            line.next().unwrap().parse::<usize>().unwrap(),
            line.next().unwrap().parse::<usize>().unwrap(),
        );
        record.entry(i).and_modify(|value| *value += v).or_insert(v);
    }
    for (k, v) in record {
        if v != 0 {
            println!("{} {}", k, v);
        }
    }
}

// use std::io::{self, Read};

// fn main() {
//     use std::collections::BTreeMap;
//     let mut input = String::new();
//     io::stdin().read_to_string(&mut input).unwrap();
//     let lines = input.lines();
//     let mut record = BTreeMap::new();
//     for line in lines.skip(1) {
//         let line = line
//             .split_whitespace()
//             .map(|x| usize::from_str_radix(x, 10).unwrap_or(0))
//             .collect::<Vec<usize>>();
//         let (i, v) = (line[0], line[1]);
//         record.entry(i).and_modify(|value| *value += v).or_insert(v);
//     }
//     for (k, v) in record {
//         if v != 0 {
//             println!("{} {}", k, v);
//         }
//     }
// }
