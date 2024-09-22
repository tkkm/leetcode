fn main() {
    println!("13. Roman to Integer");
}

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        fn add(before: u8, after: u8) -> (i32, usize) {
            // println!("{}, {}", before as char, after as char);
            match (before, after) {
                //
                (b'I', b'V') => return (4, 2),
                (b'I', b'X') => return (9, 2),
                (b'X', b'L') => return (40, 2),
                (b'X', b'C') => return (90, 2),
                (b'C', b'D') => return (400, 2),
                (b'C', b'M') => return (900, 2),
                //
                (b'I', _) => return (1, 1),
                (b'V', _) => return (5, 1),
                (b'X', _) => return (10, 1),
                (b'L', _) => return (50, 1),
                (b'C', _) => return (100, 1),
                (b'D', _) => return (500, 1),
                (b'M', _) => return (1000, 1),
                _ => unreachable!(),
            }
        }
        let letters = s.as_bytes();
        let mut sum = 0;
        let mut index = 0;

        let letter_lnegth = letters.len();
        while index < letter_lnegth {
            let after = if index == letter_lnegth - 1 {
                b'z'
            } else {
                letters[index + 1]
            };

            let (add_number, add_index) = add(letters[index], after);
            sum += add_number;
            index += add_index;
        }

        sum
    }
}

// impl Solution {
//     pub fn roman_to_int(s: String) -> i32 {
//         fn add(after: u8, before: u8) -> i32 {
//             // println!("{}, {}", before as char, letter as char);
//             match (after, before) {
//                 //
//                 (b'V' | b'X', b'I') => return -1,
//                 (b'L' | b'C', b'X') => return -10,
//                 (b'D' | b'M', b'C') => return -100,
//                 //
//                 (_, b'I') => return 1,
//                 (_, b'V') => return 5,
//                 (_, b'X') => return 10,
//                 (_, b'L') => return 50,
//                 (_, b'C') => return 100,
//                 (_, b'D') => return 500,
//                 (_, b'M') => return 1000,
//                 _ => unreachable!(),
//             }
//         }
//         let letters = s.as_bytes();

//         let mut sum = 0;
//         let mut before = letters[0];
//         for letter in letters[1..].iter() {
//             sum += add(*letter, before);
//             before = *letter
//         }

//         sum += add(b'_', before);
//         sum
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let s = "III".to_string();
        let output = 3;
        assert_eq!(Solution::roman_to_int(s), output);
    }

    #[test]
    fn test_2() {
        let s = "LVIII".to_string();
        let output = 58;
        assert_eq!(Solution::roman_to_int(s), output);
    }
    #[test]
    fn test_3() {
        let s = "MCMXCIV".to_string();
        let output = 1994;
        assert_eq!(Solution::roman_to_int(s), output);
    }
}
