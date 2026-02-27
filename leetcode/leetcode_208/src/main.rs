const MAX_CNT: usize = 100_000;

struct Trie {
    nexts: [[usize; 26]; MAX_CNT],
    pass: [usize; MAX_CNT],
    end: [usize; MAX_CNT],
    size: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Self {
            nexts: [[0; 26]; MAX_CNT],
            pass: [0; MAX_CNT],
            end: [0; MAX_CNT],
            size: 1,
        }
    }

    fn insert(&mut self, word: String) {
        let mut curr = 1;
        for c in word.as_bytes() {
            let idx = (c - b'a') as usize;
            self.pass[curr] += 1;
            if self.nexts[curr][idx] == 0 {
                self.size += 1;
                self.nexts[curr][idx] = self.size;
            }
            curr = self.nexts[curr][idx];
        }
        self.pass[curr] += 1;
        self.end[curr] += 1;
    }

    fn search(&self, word: String) -> bool {
        let mut curr = 1;
        for c in word.as_bytes() {
            let idx = (c - b'a') as usize;
            if self.nexts[curr][idx] == 0 {
                return false;
            } else {
                curr = self.nexts[curr][idx];
            }
        }
        self.end[curr] != 0
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut curr = 1;
        for c in prefix.as_bytes() {
            let idx = (c - b'a') as usize;
            if self.nexts[curr][idx] == 0 {
                return false;
            } else {
                curr = self.nexts[curr][idx];
            }
        }
        self.pass[curr] != 0 
    }
}


// use std::collections::HashMap;

// #[derive(Default)]
// struct Trie {
//     nexts:[Option<Box<Trie>>;26],
//     pass:i32,
//     end:i32,
// }

// /**
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
// impl Trie {

//     fn new() -> Self {
//         Self::default()
//     }

//     fn insert(&mut self, word: String) {
//         let mut curr = self;
//         curr.pass += 1;
//         for c in word.as_bytes() {
//             let idx = (c - b'a') as usize;
//             curr.pass += 1;
//             curr = curr.nexts[idx]
//             .get_or_insert_with(|| Box::new(Self::new()))
//             .as_mut();
//     }
//     curr.end += 1;
// }

//     fn search(&self, word: String) -> bool {
//         let mut curr = self;
//         for c in word.as_bytes() {
//                 let idx = (c - b'a') as usize;
//                 match curr.nexts[idx].as_deref() {
//                     Some(next) => curr = next,
//                     None => return false,
//                 }
//             }
//             curr.end != 0
//         }

//     fn starts_with(&self, prefix: String) -> bool {
//         let mut curr = self;
//         for c in prefix.as_bytes() {
//             let idx = (c - b'a') as usize;
//             match curr.nexts[idx].as_deref() {
//                 Some(next) => curr = next,
//                 None => return false,
//             }
//         }
//         curr.end != 0 || curr.pass != 0
//     }
// }

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

// use std::collections::HashMap;

// #[derive(Default)]
// struct Trie {
//     nexts:HashMap<char, Box<Trie>>,
//     pass:i32,
//     end:i32,
// }

// /**
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
// impl Trie {

//     fn new() -> Self {
//         Self::default()
//     }

//     fn insert(&mut self, word: String) {
//         let mut curr = self;
//         curr.pass += 1;
//         for c in word.chars() {
//             curr = curr.nexts.entry(c).or_insert_with(|| Box::new(Self::new()));
//             curr.pass += 1;
//         }
//         curr.end += 1;
//     }

//     fn search(&self, word: String) -> bool {
//         let mut curr = self;
//         for c in word.chars() {
//             match curr.nexts.get(&c) {
//                 Some(next) => curr = next.as_ref(),
//                 None => return false,
//             }
//         }
//         curr.end != 0
//     }

//     fn starts_with(&self, prefix: String) -> bool {
//         let mut curr = self;
//         for c in prefix.chars() {
//             match curr.nexts.get(&c) {
//                 Some(next) => curr = next.as_ref(),
//                 None => return false,
//             }
//         }
//         curr.end != 0 || curr.pass != 0
//     }
// }

// /**
//  * Your Trie object will be instantiated and called as such:
//  * let obj = Trie::new();
//  * obj.insert(word);
//  * let ret_2: bool = obj.search(word);
//  * let ret_3: bool = obj.starts_with(prefix);
//  */
fn main() {
    println!("Hello, world!");
}
