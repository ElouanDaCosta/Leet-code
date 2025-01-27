struct Solution;

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let res = nums.iter().copied().reduce(|a, b| a * b).unwrap();
        if res > 0 {
            return 1;
        }
        if res == 0 {
            return 0;
        }
        if res < 0 {
            return -1;
        }
        return 0;
    }
}

fn main() {
    let mut digits = vec![
        41, 65, 14, 80, 20, 10, 55, 58, 24, 56, 28, 86, 96, 10, 3, 84, 4, 41, 13, 32, 42, 43, 83,
        78, 82, 70, 15, -41,
    ];
    let result = Solution::array_sign(digits);
    println!("{:?}", result);
}
