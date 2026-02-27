struct Solution {}

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        use std::cmp::Ordering;
        let mut nums = nums;
        fn sort(nums: &mut [i32]) {
            if nums.len() < 2 {
                return ();
            }
            let pivot_value = nums[nums.len() / 2];
            let (mut lt, mut i, mut gt) = (0usize, 0usize, nums.len());
            while i < gt {
                match nums[i].cmp(&pivot_value) {
                    Ordering::Less => {
                        nums.swap(lt, i);
                        i += 1;
                        lt += 1;
                    }
                    Ordering::Equal => i += 1,
                    Ordering::Greater => {
                        gt -= 1;
                        nums.swap(gt, i);
                    }
                }
            }
            let (left_array, right_array) = nums.split_at_mut(lt);
            let (_equal_part, right_array) = right_array.split_at_mut(gt - lt);
            sort(left_array);
            sort(right_array);
        }
        sort(&mut nums);
        nums
    }
}

fn main() {}
