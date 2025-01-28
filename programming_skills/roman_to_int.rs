//https://leetcode.com/problems/roman-to-integer/description/?envType=study-plan-v2&envId=programming-skills
struct Solution;

enum Roman {
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000,
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut output: i32 = 0;
        let mut prev_value = 0;
        for c in s.chars().rev() {
            let value = match c {
                'I' => Roman::I as i32,
                'V' => Roman::V as i32,
                'X' => Roman::X as i32,
                'L' => Roman::L as i32,
                'C' => Roman::C as i32,
                'D' => Roman::D as i32,
                'M' => Roman::M as i32,
                _ => 0,
            };
            if value < prev_value {
                output -= value;
            } else {
                output += value;
            }
            prev_value = value;
        }
        return output;
    }
}

fn main() {
    let result = Solution::roman_to_int("IX".to_string());
    println!("{:?}", result);
}
