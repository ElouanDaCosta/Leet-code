struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        if nums.len() <= 1 {
            return;
        }
        let mut idx = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(idx, i);
                idx += 1;
            }
        }
    }
}

fn main() {
    let mut digits = vec![4, 3, 2, 1];
    let result = Solution::plus_one(digits);
    // println!("{:?}", result);
}
