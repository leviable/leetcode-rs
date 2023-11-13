/// Given an array of integers nums and an integer target, return indices of
/// the two numbers such that they add up to target.
///
/// You may assume that each input would have exactly one solution, and you may
/// not use the same element twice.
///
/// You can return the answer in any order.
///
///  
///
/// Example 1:
///
/// Input: nums = [2,7,11,15], target = 9
/// Output: [0,1]
/// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
///
/// Example 2:
///
/// Input: nums = [3,2,4], target = 6
/// Output: [1,2]
///
/// Example 3:
///
/// Input: nums = [3,3], target = 6
/// Output: [0,1]
///
use std::cmp::Ordering;

fn solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let i_len = nums.len() - 2;
    for i in 0..=i_len {
        let j_len = nums.len() - 1;
        for j in (i + &1)..=j_len {
            let sum = nums[i] + nums[j];
            match sum.cmp(&target) {
                Ordering::Equal => return vec![i as i32, j as i32],
                Ordering::Greater => continue,
                Ordering::Less => continue,
            }
        }
    }
    vec![0, 0]
}

#[test]
fn example_1() {
    let actual = solution(vec![2, 7, 11, 15], 9);
    let expected = [0, 1];

    assert_eq!(actual, expected);
}

#[test]
fn example_2() {
    let actual = solution(vec![3, 2, 4], 6);
    let expected = [1, 2];

    assert_eq!(actual, expected);
}

#[test]
fn example_3() {
    let actual = solution(vec![3, 3], 6);
    let expected = [0, 1];

    assert_eq!(actual, expected);
}

#[test]
fn example_4() {
    let actual = solution(vec![2, 5, 5, 11], 10);
    let expected = [1, 2];

    assert_eq!(actual, expected);
}

#[test]
fn example_5() {
    let actual = solution(vec![0, 4, 3, 0], 0);
    let expected = [0, 3];

    assert_eq!(actual, expected);
}

fn main() {
    println!("Hello, world!");
}
