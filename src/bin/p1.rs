fn main() {}

struct Solution;

// impl Solution {
//     pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//         for (i, i_val) in nums.iter().enumerate() {
//             for (j, j_val) in nums.iter().enumerate() {
//                 if i == j {
//                     continue;
//                 }
//                 if i_val + j_val == target {
//                     return vec![i as i32, j as i32];
//                 }
//             }
//         }
//         panic!()
//     }
// }

// impl Solution {
//     pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//         for (i, i_val) in nums.iter().enumerate() {
//             for (j, j_val) in nums[(i + 1)..].iter().enumerate() {
//                 if i_val + j_val == target {
//                     return vec![i as i32, (j + i + 1) as i32];
//                 }
//             }
//         }
//         panic!()
//     }
// }

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, usize> = HashMap::with_capacity(nums.len());

        for (i, &n) in nums.iter().enumerate() {
            let expected = target - n;
            match seen.get(&expected) {
                Some(&expected_i) => {
                    return vec![i as i32, expected_i as i32];
                }
                None => {
                    seen.insert(n, i);
                }
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let output = vec![0, 1];
        assert_eq!(
            Solution::two_sum(nums, target).sort(),
            output.clone().sort()
        );
    }

    #[test]
    fn test2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let output = vec![1, 2];
        assert_eq!(
            Solution::two_sum(nums, target).sort(),
            output.clone().sort()
        );
    }
    #[test]
    fn test3() {
        let nums = vec![3, 3];
        let target = 6;
        let output = vec![0, 1];
        assert_eq!(
            Solution::two_sum(nums, target).sort(),
            output.clone().sort()
        );
    }
}
