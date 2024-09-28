use std::collections::HashMap;

fn main() {
    println!("169. Majority Element")
}

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut counter: HashMap<i32, u32> = HashMap::with_capacity(100);

        for num in nums {
            *counter.entry(num).or_insert(1) += 1;
        }

        let mut max_num = 0;
        let mut max_count: u32 = 0;

        for (key, count) in counter.iter() {
            if *count > max_count {
                max_num = *key;
                max_count = *count;
            }
        }

        max_num
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let nums = vec![3, 2, 3];
        assert_eq!(Solution::majority_element(nums), 3);
    }

    #[test]
    fn test2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(Solution::majority_element(nums), 2);
    }
}
