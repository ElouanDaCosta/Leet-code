//https://leetcode.com/problems/move-zeroes/?envType=study-plan-v2&envId=programming-skills
struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        for _ in 0..nums.len() {
            for j in 0..(&nums.len() - 1) {
                if nums[j] == 0 {
                    nums.swap(j, j + 1);
                }
            }
        }
    }
}

fn main() {
    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut nums);
}
