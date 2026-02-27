pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;
    let mut sum = 0;
    let mut ans = 0;
    let mut sum_map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
    sum_map.insert(0, 1);
    for &num in &nums {
        sum += num;
        match sum_map.get(&(sum - k)) {
            Some(val) => {ans += val},
            None => (),
        }
        *sum_map.entry(sum).or_insert(0) += 1;
    }
    ans
}

fn main() {
    let test_vec = vec![1,1,1];
    println!("the result is {}", subarray_sum(test_vec, 2));
}
