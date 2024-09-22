fn main() {
    println!("35. Search Insert Position")
}
struct Solution;

// impl Solution {
//     pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
//         let max_index = nums.len() - 1;

//         let (mut left, mut right) = (0, max_index as i32);

//         loop {
//             if right - left <= 1 {
//                 if target > nums[right as usize] {
//                     return right + 1;
//                 } else if target > nums[left as usize] {
//                     return right;
//                 } else {
//                     return left;
//                 }
//             }

//             let middle = (left + right) / 2;
//             if target >= nums[middle as usize] {
//                 left = middle;
//             } else {
//                 right = middle;
//             }
//         }
//     }
// }

use std::cmp::Ordering;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let max_index = nums.len() - 1;

        let (mut left, mut right) = (0, max_index as i32);

        while left <= right {
            let middle = (left + right) / 2;

            match nums[middle as usize].cmp(&target) {
                Ordering::Equal => return middle,
                Ordering::Greater => right = middle - 1,
                Ordering::Less => left = middle + 1,
            }
        }

        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }
}
