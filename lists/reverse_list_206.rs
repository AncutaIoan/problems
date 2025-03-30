// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
/*
Given the head of a singly linked list, reverse the list, and return the reversed list.
*/
use std::mem::swap;

impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        while let Some(mut current) = head {
            head = current.next.take(); // Move the head to the next node
            current.next = prev; // Reverse the link
            prev = Some(current); // Move prev forward
        }
        prev
    }
}
