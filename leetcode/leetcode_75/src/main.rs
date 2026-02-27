struct Solution {}

// impl Solution {
//     pub fn sort_colors(nums: &mut Vec<i32>) {
//         if nums.is_empty(){
//             return ();
//         }
//         let (mut lt, mut gt) = (0usize, nums.len() - 1);
//         let mut i = 0;
//         while i <= gt && gt > 0 {
//             if nums[i] == 0 {
//                 nums.swap(i, lt);
//                 lt += 1;
//                 i += 1;
//             } else if nums[i] == 2 {
//                 nums.swap(i, gt);
//                 gt -= 1;
//             } else {
//                 i += 1;
//             }
//         }
//     }
// }

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.is_empty(){
            return ();
        }
        let (mut lt, mut gt) = (0usize, nums.len());
        let mut i = 0;
        while i < gt {
            if nums[i] == 0 {
                nums.swap(i, lt);
                lt += 1;
                i += 1;
            } else if nums[i] == 2 {
                gt -= 1;
                nums.swap(i, gt);
            } else {
                i += 1;
            }
        }
    }
}


fn main() {
    println!("Hello, world!");
}
