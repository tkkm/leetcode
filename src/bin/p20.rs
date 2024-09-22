fn main() {
    println!("20. Valid Parentheses")
}

struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        const OPEN_BRACKETS: [u8; 3] = [b'(', b'{', b'['];

        let s = s.as_bytes();
        let mut deque = VecDeque::with_capacity(s.len() / 2);

        for char in s {
            if OPEN_BRACKETS.contains(char) {
                deque.push_front(char);
            } else {
                let poped_char = match deque.pop_front() {
                    Some(c) => c,
                    None => return false,
                };

                match (poped_char, char) {
                    (b'(', b')') => {}
                    (b'{', b'}') => {}
                    (b'[', b']') => {}
                    _ => return false,
                }
            }
        }

        return deque.is_empty();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::is_valid("(]".to_string()), false);
    }
}
