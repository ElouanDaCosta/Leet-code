//https://leetcode.com/problems/baseball-game/?envType=study-plan-v2&envId=programming-skills
struct Solution;

impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut score: Vec<i32> = Vec::new();
        let mut previous_int: i32 = 0;
        for i in 0..operations.len() {
            println!("operation vector {}", operations[i]);
            println!("previous_int {:?}", previous_int);
            if operations[i] == "C" {
                score.pop();
                if !score.is_empty() {
                    previous_int = *score.last().unwrap();
                }
            } else if operations[i] == "D" {
                score.push(previous_int * 2);
                previous_int = *score.last().unwrap();
            } else if operations[i] == "+" {
                score.push(score[score.len() - 1] + score[score.len() - 2]);
                previous_int = *score.last().unwrap();
            } else {
                score.push(operations[i].parse::<i32>().unwrap());
                previous_int = *score.last().unwrap();
            }
            println!("score vector {:?}", score);
        }
        let sum = score.iter().sum();
        return sum;
    }
}

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn main() {
    let nums: Vec<String> =
        vec_of_strings!["61", "-50", "65", "+", "D", "-99", "-58", "88", "19", "-11"];
    let result = Solution::cal_points(nums);
    println!("{:?}", result);
}
