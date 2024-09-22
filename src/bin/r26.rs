// [0,0,1,1,1,2,2,3,3,4]
// 0, 0
// 1, 1
// 2, 1

fn main() {
    println!("26. Remove Duplicates from Sorted Array")
}

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut insert_index = 0;

        for loop_index in 0..nums.len() {
            let num = nums[loop_index];

            if nums[insert_index] != num {
                insert_index += 1;
                nums[insert_index] = num;
            }

            // println!("{} {} {} {:?}", num, insert_index, loop_index, nums);
        }
        (insert_index + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let mut nums = vec![1, 1, 2];
        let num = Solution::remove_duplicates(&mut nums);
        assert_eq!(num, 2);
        assert_eq!(nums[..2], vec![1, 2]);
    }

    #[test]
    fn test2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let num = Solution::remove_duplicates(&mut nums);
        assert_eq!(num, 5);
        assert_eq!(nums[..5], vec![0, 1, 2, 3, 4]);
    }
}
