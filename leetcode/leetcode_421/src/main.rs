struct Solution {}

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 2],
}

impl Trie {
    fn new() -> Self {
        Trie::default()
    }

    fn insert(&mut self, num: i32, lz: i32) {
        let mut curr = self;
        for i in (0..32 - lz).rev() {
            let bit = ((num >> i) & 1) as usize;
            curr = curr.children[bit].get_or_insert_with(|| Box::new(Self::new()));   
        }
    }

    fn get_ORmax(&self, num: i32, lz: i32) -> i32 {
        let mut curr = self;
        let mut max = 0;
        for i in (0..32 - lz).rev() {
            let bit = ((num >> i) & 1) as usize;
            if let Some(node) = curr.children[1- bit].as_deref() {
                max |= 1 << i;
                curr = node;
            } else {
                curr = curr.children[bit].as_deref().unwrap();
            }
        } 
        max
    }
}

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let lz = nums.iter().copied().max().unwrap_or(0).leading_zeros() as i32;
        let mut root = Trie::new();
        for num in nums.iter() {
            root.insert(*num, lz);
        }
        let mut ans = 0;
        for num in nums.iter() {
            ans = ans.max(root.get_ORmax(*num, lz));
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
