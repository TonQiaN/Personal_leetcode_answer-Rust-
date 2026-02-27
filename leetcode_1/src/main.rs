use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_index_map:HashMap<&i32, i32> = HashMap::with_capacity(nums.len());
    for (num, i) in nums.iter().zip(0..) {
        let answer = target - num;
        match num_index_map.get(&answer) {
            Some(j) => return vec![*j, i],
            None => {num_index_map.insert(nums.get(i as usize).unwrap(), i);},
        }
    }
    vec![]
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum(nums, target);
    println!("The result is {:?}", result);
}
