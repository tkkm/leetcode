fn main() {
    println!("14. Longest Common Prefix")
}

struct Solution;

// use std::collections::HashSet;

// impl Solution {
//     pub fn longest_common_prefix(strs: Vec<String>) -> String {
//         let min_length = strs.iter().map(|str| str.len()).min().unwrap();

//         let str_bytes = strs.iter().map(|str| str.as_bytes());
//         let str_bytes: Vec<_> = str_bytes.collect();

//         let mut common_predix = String::with_capacity(min_length);
//         for index in 0..min_length {
//             if str_bytes
//                 .iter()
//                 .map(|&str| str[index])
//                 .collect::<HashSet<_>>()
//                 .len()
//                 == 1
//             {
//                 common_predix.push(str_bytes[0][index] as char);
//             } else {
//                 break;
//             }
//         }
//         common_predix
//     }
// }

// impl Solution {
//     pub fn longest_common_prefix(strs: Vec<String>) -> String {
//         let min_length = strs.iter().map(|str| str.len()).min().unwrap();

//         let str_bytes = strs.iter().map(|str| str.as_bytes());
//         let str_bytes: Vec<_> = str_bytes.collect();

//         let mut common_predix = String::with_capacity(min_length);
//         'outer: for index in 0..min_length {
//             let char = str_bytes[0][index];
//             for &str_byte in str_bytes.iter().skip(1) {
//                 if char != str_byte[index] {
//                     break 'outer;
//                 }
//             }
//             common_predix.push(char as char)
//         }

//         common_predix
//     }
// }

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let min_length = strs.iter().map(|str| str.len()).min().unwrap();

        let str_bytes = strs.iter().map(|str| str.as_bytes());
        let str_bytes: Vec<_> = str_bytes.collect();

        for index in 0..min_length {
            let char = str_bytes[0][index];
            for &str_byte in str_bytes.iter().skip(1) {
                if char != str_byte[index] {
                    return strs[0][..index].to_string();
                }
            }
        }

        strs[0][..min_length].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];

        assert_eq!(Solution::longest_common_prefix(strs), "fl");
    }
    #[test]
    fn test2() {
        let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];

        assert_eq!(Solution::longest_common_prefix(strs), "");
    }
}
