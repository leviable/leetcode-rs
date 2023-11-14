/// You are given the heads of two sorted linked lists list1 and list2.
///
/// Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.
///
/// Return the head of the merged linked list.
///
///  
///
/// Example 1:
///
///
/// Input: list1 = [1,2,4], list2 = [1,3,4]
/// Output: [1,1,2,3,4,4]
///
/// Example 2:
///
/// Input: list1 = [], list2 = []
/// Output: []
///
/// Example 3:
///
/// Input: list1 = [], list2 = [0]
/// Output: [0]
///  
///
/// Constraints:
///
/// The number of nodes in both lists is in the range [0, 50].
/// -100 <= Node.val <= 100
/// Both list1 and list2 are sorted in non-decreasing order.
/// // Definition for singly-linked list.
use std::cmp::Ordering;
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1 == None && list2 == None {
            return None;
        } else if list1 == None {
            return list2;
        } else if list2 == None {
            return list1;
        }

        let mut list1 = list1.unwrap();
        let mut list2 = list2.unwrap();

        match list1.val.cmp(&list2.val) {
            Ordering::Equal | Ordering::Less => {
                let mut node = list1;
                let old = node.next;
                node.next = Solution::merge_two_lists(old, Some(list2));
                Some(node)
            }
            Ordering::Greater => {
                let mut node = list2;
                let old = node.next;
                node.next = Solution::merge_two_lists(Some(list1), old);
                Some(node)
            }
        }
    }
}
