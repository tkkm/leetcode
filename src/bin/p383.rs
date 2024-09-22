fn main() {}
struct Solution;

// use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        const NUM_LETTERS: usize = (b'z' - b'a' + 1) as usize;
        const FIRST_LETTER: u8 = b'a';

        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut magazine_lettres = [0; NUM_LETTERS];
        magazine.bytes().for_each(|letter| {
            magazine_lettres[(letter - FIRST_LETTER) as usize] += 1;
        });

        for letter in ransom_note.bytes() {
            let index = (letter - FIRST_LETTER) as usize;
            match magazine_lettres[index] {
                0 => return false,
                _ => magazine_lettres[index] -= 1,
            }
        }

        return true;
    }
}

// impl Solution {
//     pub fn can_construct(ransom_note: String, magazine: String) -> bool {
//         let mut note_letters = HashMap::with_capacity(26);
//         for letter in ransom_note.chars() {
//             note_letters
//                 .entry(letter)
//                 .and_modify(|count| *count += 1)
//                 .or_insert(1);
//         }

//         for letter in magazine.chars() {
//             if let Some(value) = note_letters.get(&letter) {
//                 if value <= &1 {
//                     note_letters.remove(&letter);
//                 } else {
//                     note_letters.insert(letter, value - 1);
//                 }
//             }

//             if note_letters.is_empty() {
//                 return true;
//             }
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ransom_note = "a".to_string();
        let magazine = "b".to_string();
        assert_eq!(Solution::can_construct(ransom_note, magazine), false)
    }

    #[test]
    fn test2() {
        let ransom_note = "aa".to_string();
        let magazine = "ab".to_string();
        assert_eq!(Solution::can_construct(ransom_note, magazine), false)
    }

    #[test]
    fn test3() {
        let ransom_note = "aa".to_string();
        let magazine = "aab".to_string();
        assert_eq!(Solution::can_construct(ransom_note, magazine), true)
    }
}
