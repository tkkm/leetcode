fn main() {
    println!("189. Rotate Array");
}

struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let n_rotate = k as usize % n;
        // nums.rotate_right(n_rotate);

        // let cloned = nums.iter().cloned().collect::<Vec<i32>>();
        // for i in 0..n {
        //     nums[(i + n_rotate) % n] = cloned[i];
        // }

        nums.reverse();

        nums[..n_rotate].reverse();
        nums[n_rotate..].reverse();
    }
}

// [1, 2, 3, 4, 5, 6, 7]
// [7, 6, 5, 4, 3, 2, 1]
// [5, 6, 7, 1, 2, 3, 4]

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn test2() {
        let mut nums = vec![-1, -100, 3, 99];
        Solution::rotate(&mut nums, 2);
        assert_eq!(nums, vec![3, 99, -1, -100]);
    }
}
