fn main() {
    println!("Hello, world!");
    let nums = vec![1, 2, 3, 4];

    let result = Solution::running_sum(nums);
    println!("{:?}", result);
}

struct Solution;
type R = Vec<i32>;

impl Solution {
    pub fn running_sum(nums: R) -> R {
        let mut temp_num = 0;
        nums.into_iter()
            .map(|num| {
                temp_num += num;
                temp_num
            })
            .collect()
    }
}
