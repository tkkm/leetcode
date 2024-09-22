fn main() {
    println!("1672. Richest Customer Wealth!");
}

struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let r = (1..=n)
            .map(|i| match (i % 3, i % 5) {
                (0, 0) => "FizzBuzz".to_string(),
                (0, _) => "Fizz".to_string(),
                (_, 0) => "Buzz".to_string(),
                _ => i.to_string(),
            })
            .collect();

        r

        // let mut results = Vec::with_capacity(n as usize);
        // for i in 1..=n {
        //     let mut r = String::from("");
        //     if i % 3 == 0 {
        //         r.push_str("Fizz");
        //     }

        //     if i % 5 == 0 {
        //         r.push_str("Buzz");
        //     }

        //     if r == "" {
        //         r = i.to_string();
        //     }
        //     results.push(r);
        // }

        // results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = 3;
        let expected = vec!["1", "2", "Fizz"];
        assert_eq!(Solution::fizz_buzz(input), expected);
    }

    #[test]
    fn test2() {
        let input = 5;
        let expected = vec!["1", "2", "Fizz", "4", "Buzz"];
        assert_eq!(Solution::fizz_buzz(input), expected);
    }

    #[test]
    fn test_3() {
        let input = 15;
        let expected = vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz",
        ];
        assert_eq!(Solution::fizz_buzz(input), expected);
    }
}
