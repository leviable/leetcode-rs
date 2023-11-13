/// Given an integer x, return true if x is a
/// palindrome
/// , and false otherwise.
///
///  
///
/// Example 1:
///
/// Input: x = 121
/// Output: true
/// Explanation: 121 reads as 121 from left to right and from right
/// to left.
///
/// Example 2:
///
/// Input: x = -121
/// Output: false
/// Explanation: From left to right, it reads -121. From right
/// to left, it becomes 121-. Therefore it is not a palindrome.
///
/// Example 3:
///
/// Input: x = 10
/// Output: false
/// Explanation: Reads 01 from right to left. Therefore it is
/// not a palindrome.
///  
///
/// Constraints:
///
/// -231 <= x <= 231 - 1
fn solution(x: i32) -> bool {
    let forward = x.to_string();
    let reverse = forward.clone().chars().rev().collect::<String>();
    forward == reverse
}

#[test]
fn example_1() {
    let actual = solution(121);
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn example_2() {
    let actual = solution(-121);
    let expected = false;

    assert_eq!(actual, expected);
}

#[test]
fn example_3() {
    let actual = solution(10);
    let expected = false;

    assert_eq!(actual, expected);
}

fn main() {
    println!("Hello, world!");
}
