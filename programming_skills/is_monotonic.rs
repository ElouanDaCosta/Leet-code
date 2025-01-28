//https://leetcode.com/problems/monotonic-array/?envType=study-plan-v2&envId=programming-skills
struct Solution;

impl Solution {
    pub fn is_monotonic(mut nums: Vec<i32>) -> bool {
        let mut is_increasing: bool = false;
        let mut is_decreasing: bool = false;
        for i in 1..nums.len() {
            println!("{} {}", is_increasing, is_decreasing);
            if nums[i] < nums[i - 1] {
                is_decreasing = true;
            }
            if nums[i] > nums[i - 1] {
                is_increasing = true;
            }
            if is_decreasing == true && is_increasing == true {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut nums = vec![1, 3, 2];
    let result = Solution::is_monotonic(nums);
    println!("{}", result);
}
