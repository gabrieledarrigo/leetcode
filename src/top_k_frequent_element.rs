// Given an integer array nums and an integer k, return the k most frequent elements.
// You may return the answer in any order.

// Example 1:
// Input: nums = [1,1,1,2,2,3], k = 2
// Output: [1,2]

// Example 2:
// Input: nums = [1], k = 1
// Output: [1]

// Follow up: Your algorithm's time complexity must be better than O(n log n), where n is the array's size.

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn top_k_frequent(numbers: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for number in numbers {
            map.entry(number)
                .and_modify(|frequency| *frequency += 1)
                .or_default();
        }

        let mut vec: Vec<(i32, i32)> = map.into_iter().collect();
        vec.sort_by(|(_, a), (_, b)| b.cmp(a));
        vec.truncate(k as usize);

        vec.into_iter().map(|(element, _)| element).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_top_k_frequent() {
        assert_eq!(
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 2]
        );
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }
}
