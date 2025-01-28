/https://leetcode.com/problems/to-lower-case/?envType=study-plan-v2&envId=programming-skills
struct Solution;

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        return s.to_lowercase();
    }
}

fn main() {
    let result = Solution::to_lower_case("Hello".to_string());
    println!("{:?}", result);
}
