fn main() {
    println!("{:?}", (1..1).map(|x| { x }).collect::<Vec<i32>>());
    println!("27. Remove Element");
}

struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        // nums.retain(|x| *x != val);
        // nums.len() as i32
        let mut i = 0;

        for ii in 0..nums.len() {
            let num = nums[ii];
            nums[i] = num;

            if num != val {
                i += 1;
            }
        }
        i as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let mut nums = vec![3, 2, 2, 3];

        let out_num = Solution::remove_element(&mut nums, 3) as u16;

        assert_eq!(out_num, 2);
        assert_eq!(nums[..2], vec![2, 2]);
    }
    #[test]
    fn test2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];

        let out_num = Solution::remove_element(&mut nums, 2) as u16;

        assert_eq!(out_num, 5);
        assert_eq!(nums[..5], vec![0, 1, 4, 0, 3]);
    }
}
