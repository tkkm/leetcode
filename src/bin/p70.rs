fn main() {
    println!("{:?}", (1..1).map(|x| { x }).collect::<Vec<i32>>());
    println!("70. Climbing Stairs");
}

struct Solution;

// impl Solution {
//     pub fn climb_stairs(n: i32) -> i32 {
//         type U = u128;

//         let is_odd = (n & 1) == 1;

//         let mut count = 1;
//         for (m, _n) in (0..n).rev().zip(1..n) {
//             let n = _n.min(m - _n) as U;
//             count += ((m as U - n + 1)..=m as U).product::<U>() / (1..=n).product::<U>();

//             let diff = m - _n;
//             if diff <= 0 || (is_odd && diff <= 1) {
//                 break;
//             }
//         }

//         return count as i32;
//     }
// }

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        } else if n == 2 {
            return 2;
        }

        let mut steps: [i32; 45] = [0; 45];
        steps[0] = 1;
        steps[1] = 2;

        // for index in 2..n as usize {
        let mut index: usize = 2;
        while index < n as usize {
            steps[index] = steps[index - 2] + steps[index - 1];
            index += 1;
        }

        return steps[(n - 1) as usize];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::climb_stairs(4), 5);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::climb_stairs(5), 8);
    }
}
