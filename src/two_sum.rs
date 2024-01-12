use std::collections::HashMap;

/// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
///
/// You may assume that each input would have exactly one solution, and you may not use the same element twice.
///
/// You can return the answer in any order.
///
/// Example 1:
///
/// Input: nums = [2,7,11,15], target = 9
/// Output: [0,1]
/// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
/// Example 2:
///
/// Input: nums = [3,2,4], target = 6
/// Output: [1,2]
/// Example 3:
///
/// Input: nums = [3,3], target = 6
/// Output: [0,1]
///
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut answer: Vec<i32> = vec![];

    let len = nums.len();

    for i in 0..len {
        for k in (i + 1)..len {
            if (nums[i] + nums[k]) == target {
                answer.push(i as i32);
                answer.push(k as i32);
            }
        }
    }

    answer
}

pub fn two_sum_with_index(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut indices = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(&index) = indices.get(&complement) {
            return vec![index as i32, i as i32];
        }

        indices.insert(num, i);
    }

    vec![]
}

mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
        assert_eq!(
            two_sum(vec![1, 4, 7, 9, 3, 5, 7, 6, 12, 0, 11], 20),
            vec![3, 10]
        );
    }

    #[test]
    fn test_two_sum_with_index() {
        // Complement: 7; Insert k:2, v:0
        // Complement: 2; It matches: the answer is [0, 1];
        assert_eq!(two_sum_with_index(vec![2, 7, 11, 15], 9), vec![0, 1]);

        // Complement: 3; Insert k:3, v:0
        // Complement: 4; Insert k:2, v:1
        // Complement: 2; It matches: the answer is [1, 2];
        assert_eq!(two_sum_with_index(vec![3, 2, 4], 6), vec![1, 2]);

        // Complement: 3; Insert k:3, v:0
        // Complement: 3; It matches: the answer is [0, 1];
        assert_eq!(two_sum_with_index(vec![3, 3], 6), vec![0, 1]);

        // Complement: 19; Insert k:1, v:0
        // Complement: 16; Insert k:4, v:1
        // Complement: 13; Insert k:7, v:2
        // Complement: 11; Insert k:9, v:3
        // Complement: 17; Insert k:3, v:4
        // Complement: 15; Insert k:5, v:5
        // Complement: 13; Insert k:7, v:6
        // Complement: 14; Insert k:6, v:7
        // Complement: 8; Insert k:12, v:8
        // Complement: 20; It matches: the answer is [3, 10];
        assert_eq!(
            two_sum_with_index(vec![1, 4, 7, 9, 3, 5, 7, 6, 12, 0, 11], 20),
            vec![3, 10]
        );
    }
}
