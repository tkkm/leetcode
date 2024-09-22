fn main() {
    println!("1672. Richest Customer Wealth!");
}

struct Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut count = 0;
        let mut num = num;

        // (1)
        // while num > 0 {
        //     count += 1;

        //     match num % 2 {
        //         0 => num /= 2,
        //         _ => num -= 1,
        //     }
        // }

        // (2)
        while num > 0 {
            count += num & 1;

            num >>= 1;
            if num > 0 {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = 14;
        let expected = 6;
        assert_eq!(Solution::number_of_steps(input), expected);
    }

    #[test]
    fn test2() {
        let input = 8;
        let expected = 4;
        assert_eq!(Solution::number_of_steps(input), expected);
    }

    #[test]
    fn test_3() {
        let input = 123;
        let expected = 12;
        assert_eq!(Solution::number_of_steps(input), expected);
    }
}
