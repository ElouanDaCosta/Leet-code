use std::iter::FromIterator;
use std::iter::Iterator;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let s_slice: &str = &s[..];
        let t_slice: &str = &t[..];

        //s
        let mut chars_s: Vec<char> = s_slice.chars().collect();
        chars_s.sort_by(|a, b| b.cmp(a));

        let s = String::from_iter(chars_s);

        //t
        let mut chars_t: Vec<char> = t_slice.chars().collect();
        chars_t.sort_by(|a, b| b.cmp(a));

        let t = String::from_iter(chars_t);
        if s == t {
            return true;
        }
        return false;
    }
}

fn main() {
    let result = Solution::is_anagram("anagram".to_string(), "nagaram".to_string());
    println!("Is anagram: {}", result);
}
