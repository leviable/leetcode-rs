/// Write a function to find the longest common prefix string amongst an array of strings.
///
/// If there is no common prefix, return an empty string "".
///
///  
///
/// Example 1:
///
/// Input: strs = ["flower","flow","flight"]
/// Output: "fl"
///
/// Example 2:
///
/// Input: strs = ["dog","racecar","car"]
/// Output: ""
/// Explanation: There is no common prefix among the input strings.
///  
///
/// Constraints:
///
/// 1 <= strs.length <= 200
/// 0 <= strs[i].length <= 200
/// strs[i] consists of only lowercase English letters.
fn solution(strs: Vec<String>) -> String {
    let mut answer = String::new();
    if strs[0] == "".to_string() {
        return answer;
    }
    let max = strs.iter().fold(
        200,
        |acc, word| {
            if word.len() < acc {
                word.len()
            } else {
                acc
            }
        },
    );
    for idx in 0..max {
        let mut letters = String::new();
        for word in strs.clone() {
            letters.push(word.chars().collect::<Vec<_>>()[idx]);
        }
        let letter = letters.clone().chars().next().unwrap();
        if letters
            .chars()
            .fold(0, |acc, elem| if elem == letter { acc + 1 } else { acc })
            != letters.len()
        {
            break;
        } else {
            answer.push(letter);
        }
    }

    answer
}

#[test]
fn example_1() {
    let actual = solution(vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ]);
    let expected = "fl";

    assert_eq!(actual, expected);
}

#[test]
fn example_2() {
    let actual = solution(vec![
        "dog".to_string(),
        "racecar".to_string(),
        "car".to_string(),
    ]);
    let expected = "";

    assert_eq!(actual, expected);
}

#[test]
fn example_3() {
    let actual = solution(vec!["".to_string()]);
    let expected = "";

    assert_eq!(actual, expected);
}

#[test]
fn example_4() {
    let actual = solution(vec!["a".to_string()]);
    let expected = "a";

    assert_eq!(actual, expected);
}

#[test]
fn example_5() {
    let actual = solution(vec!["ab".to_string(), "a".to_string()]);
    let expected = "a";

    assert_eq!(actual, expected);
}

fn main() {
    println!("Hello, world!");
}
