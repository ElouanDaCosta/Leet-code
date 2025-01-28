//https://leetcode.com/problems/length-of-last-word/submissions/1523605277/?envType=study-plan-v2&envId=programming-skills
struct Solution;

use std::convert::TryInto;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let split = s.trim().split(' ');

        let output = split.last().expect("REASON").chars().count();
        return output.try_into().unwrap();
    }
}

fn main() {
    let result = Solution::length_of_last_word("   fly me   to   the moon  ".to_string());
    println!("{:?}", result);
}
