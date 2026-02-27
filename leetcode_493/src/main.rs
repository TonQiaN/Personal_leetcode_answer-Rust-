fn calculate_reverse_pairs(nums: &mut Vec<i32>, left: usize, right: usize) -> i32 {
    if left >= right {
        return 0;
    }
    let mid = left + (right - left) / 2;
    calculate_reverse_pairs(nums, left, mid)
        + calculate_reverse_pairs(nums, mid + 1, right)
        + merge(nums, left, mid, right)
}

fn merge(nums: &mut Vec<i32>, left: usize, mid: usize, right: usize) -> i32 {
    let mut ans = 0;
    let mut curr_sum = 0;
    let mut l_ptr = left;
    let mut r_ptr = mid + 1;
    while l_ptr <= mid && r_ptr <= right {
        if (nums[l_ptr] as i64) > 2 * (nums[r_ptr] as i64) {
            curr_sum += 1;
            r_ptr += 1;
        } else {
            ans += curr_sum;
            l_ptr += 1;
        }
    }
    while l_ptr <= mid {
        ans += curr_sum;
        l_ptr += 1;
    }

    let mut temp_vec = Vec::new();
    l_ptr = left;
    r_ptr = mid + 1;
    while l_ptr <= mid && r_ptr <= right {
        if nums[l_ptr] < nums[r_ptr] {
            temp_vec.push(nums[l_ptr]);
            l_ptr += 1;
        } else {
            temp_vec.push(nums[r_ptr]);
            r_ptr += 1;
        }
    }
    while l_ptr <= mid {
        temp_vec.push(nums[l_ptr]);
        l_ptr += 1;
    }
    while r_ptr <= right {
        temp_vec.push(nums[r_ptr]);
        r_ptr += 1;
    }

    for i in left..=right {
        nums[i] = temp_vec[i - left];
    }

    ans
}
fn main() {
    let mut test_vec = vec![1, 3, 2, 3, 1];
    let n = test_vec.len();
    println!("{}", calculate_reverse_pairs(&mut test_vec, 0, n - 1));
}
