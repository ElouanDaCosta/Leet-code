//https://leetcode.com/problems/robot-return-to-origin/?envType=study-plan-v2&envId=programming-skills
struct Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let split = moves.split("");
        for c in split {
            println!("{:?}", c);
            match c {
                "U" => y += 1,
                "D" => y -= 1,
                "L" => x -= 1,
                "R" => x += 1,
                _ => {}
            }
        }
        if x == 0 && y == 0 {
            return true;
        }
        false
    }
}

fn main() {
    let result = Solution::judge_circle("LL".to_string());
    println!("{:?}", result);
}

// There is a robot starting at the position (0, 0),
// the origin, on a 2D plane. Given a sequence of its moves, judge
// if this robot ends up at (0, 0) after it completes its moves.

//TODO
// (0,0) == x y
// up down modify y, left right modify y
// so at the end if the coordinates is not 0 0 return false
