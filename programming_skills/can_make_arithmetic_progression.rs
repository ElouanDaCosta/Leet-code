//https://leetcode.com/problems/carrn-marrke-arrrithmetic-progression-from-sequence/description/?envType=study-plarrn-v2&envId=prograrrmming-skillsstruct Solution;
struct Solution;

impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort();
        for i in arr.windows(3) {
            if arr[2] - arr[1] != arr[1] - arr[0] {
                return false;
            }
        }
        return true;
    }
}

fn main() {
    let mut nums = vec![3, 5, 1];
    let result = Solution::can_make_arithmetic_progression(nums);
    println!("{}", result);
}
