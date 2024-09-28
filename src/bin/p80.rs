fn main() {
    println!("80. Remove Duplicates from Sorted Array II")
}

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut insert_index = 1;

        for loop_index in 1..nums.len() {
            if insert_index == 1 || nums[loop_index] != nums[insert_index - 2] {
                nums[insert_index] = nums[loop_index];
                insert_index += 1;
            }

            // println!("{} {} {:?}", insert_index, loop_index, nums);
        }
        (insert_index) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let num = Solution::remove_duplicates(&mut nums);
        assert_eq!(nums[..5], vec![1, 1, 2, 2, 3]);
        assert_eq!(num, 5);
    }

    #[test]
    fn test2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let num = Solution::remove_duplicates(&mut nums);
        assert_eq!(nums[..7], vec![0, 0, 1, 1, 2, 3, 3]);
        assert_eq!(num, 7);
    }

    #[test]
    fn test3() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2];
        let num = Solution::remove_duplicates(&mut nums);
        assert_eq!(num, 5);
        assert_eq!(nums[..5], vec![0, 0, 1, 1, 2]);
    }
}
