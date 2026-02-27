use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::VecDeque;
        fn my_push(my_queue: &mut VecDeque<i32>, num: i32) {
            while let Some(&last) = my_queue.back() {
                if last < num {
                    my_queue.pop_back();
                } else {
                    break;
                }
            }
            my_queue.push_back(num);
        }

        fn my_pop(my_queue: &mut VecDeque<i32>, num: i32) {
            if let Some(&largest) = my_queue.front() {
                if largest == num {
                    my_queue.pop_front();
                }
            }
        }

        let mut my_queue = VecDeque::new();
        let mut ans = vec![];
        for i in 0..nums.len() {
            my_push(&mut my_queue, nums[i]);
            match (i + 1).cmp(&(k as usize)) {
                Ordering::Less => continue,
                Ordering::Equal => ans.push(my_queue[0]),
                Ordering::Greater => {
                    my_pop(&mut my_queue, nums[i - k as usize]);
                    ans.push(my_queue[0]);
                }

            }
            // if i + 1 < k as usize {
            //     continue;
            // } else if i + 1 == k as usize{
            //     ans.push(my_queue[0]);
            // } else {
            //     my_pop(&mut my_queue, nums[i - k as usize]);
            //     ans.push(my_queue[0]);
            // }
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
