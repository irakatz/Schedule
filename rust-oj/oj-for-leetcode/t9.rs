//206.反转链表
//https://leetcode-cn.com/problems/reverse-linked-list/
//翻转一个链表
//rust
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
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {return None;}
        let mut curr: Option<Box<ListNode>> = head.clone();
        let mut nxt=curr.clone().unwrap().next;
        if nxt.is_none(){return head.clone();}
        let mut prev: Option<Box<ListNode>> = Solution::reverse_list(nxt);
        curr.unwrap().next.unwrap().next=curr.clone();
        nxt=None;
        return prev;
    }
}
//非rust
/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) { val = x; }
 * }
 */
// class Solution {
// public ListNode reverseList(ListNode head) {
// if (head == null){return null;}
//
// ListNode dummy = new ListNode(-1);
// dummy.next = head;
// ListNode prev = dummy.next;
// ListNode pCur = prev.next;
// while (pCur != null) {
// prev.next = pCur.next;
// pCur.next = dummy.next;
// dummy.next = pCur;
// pCur = prev.next;
// }
// return dummy.next;
// }
// }