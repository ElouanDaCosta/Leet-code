//https://leetcode.com/problems/repeated-substring-pattern/?envType=study-plan-v2&envId=programming-skills
struct Solution;

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let concat = s.clone() + &s;
        let mut chars = concat.chars();
        chars.next();
        chars.next_back();
        chars.as_str();
        let r: String = chars.into_iter().collect();
        if r.find(&s) != None {
            return true;
        }
        return false;
    }
}

fn main() {
    let result = Solution::repeated_substring_pattern("aba".to_string());
    println!("{:?}", result);
}
