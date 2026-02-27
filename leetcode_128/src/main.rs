
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    use std::cmp::max;
    let nums_set:HashSet<_> = nums.into_iter().collect();
    let mut ans = 0;
    for &num in &nums_set {
        if !nums_set.contains(&(num - 1)) {
            let count = (num..).take_while(|x| nums_set.contains(x)).count();
            ans = max(count, ans);
        }
    }
    ans as i32
}

fn main() {
    let test_case = vec![100,4,200,1,3,2];
    let result = longest_consecutive(test_case);
    println!("The result is {result}");
}
