/// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
///
/// An input string is valid if:
///
/// Open brackets must be closed by the same type of brackets.
/// Open brackets must be closed in the correct order.
/// Every close bracket has a corresponding open bracket of the same type.
///  
///
/// Example 1:
///
/// Input: s = "()"
/// Output: true
///
/// Example 2:
///
/// Input: s = "()[]{}"
/// Output: true
//
/// Example 3:
///
/// Input: s = "(]"
/// Output: false
///  
///
/// Constraints:
///
/// 1 <= s.length <= 104
/// s consists of parentheses only '()[]{}'.
use std::collections::HashMap;

fn solution(s: String) -> bool {
    let hm: HashMap<char, char> = [(')', '('), (']', '['), ('}', '{')]
        .iter()
        .cloned()
        .collect();
    let mut stack = String::new();
    for p in s.chars() {
        match p {
            '(' | '[' | '{' => stack.push(p),
            ')' | ']' | '}' => {
                let paren = match stack.pop() {
                    Some(paren) => paren,
                    None => return false,
                };
                if paren != *hm.get(&p).unwrap() {
                    return false;
                }
            }
            _ => unreachable!(),
        }
    }
    stack.len() == 0
}

#[test]
fn example_1() {
    let actual = solution("()".to_string());
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn example_2() {
    let actual = solution("()[]{}".to_string());
    let expected = true;

    assert_eq!(actual, expected);
}

#[test]
fn example_3() {
    let actual = solution("(]".to_string());
    let expected = false;

    assert_eq!(actual, expected);
}

#[test]
fn example_4() {
    let actual = solution("(".to_string());
    let expected = false;

    assert_eq!(actual, expected);
}

#[test]
fn example_5() {
    let actual = solution(")".to_string());
    let expected = false;

    assert_eq!(actual, expected);
}

fn main() {
    println!("Hello, world!");
}
