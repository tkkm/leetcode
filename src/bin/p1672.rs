fn main() {
    println!("1672. Richest Customer Wealth!");
}

struct Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut maximum_wealth = 0;

        for account in accounts {
            maximum_wealth = maximum_wealth.max(account.into_iter().sum::<i32>());
        }

        maximum_wealth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = vec![vec![1, 2, 3], vec![3, 2, 1]];
        let expected = 6;
        assert_eq!(Solution::maximum_wealth(input), expected);
    }

    #[test]
    fn test2() {
        let input = vec![vec![1, 5], vec![7, 3], vec![3, 5]];
        let expected = 10;
        assert_eq!(Solution::maximum_wealth(input), expected);
    }

    #[test]
    fn test_3() {
        let input = vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]];
        let expected = 17;
        assert_eq!(Solution::maximum_wealth(input), expected);
    }
}
