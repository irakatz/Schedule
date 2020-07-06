//翻转链表
//https://leetcode-cn.com/problems/reverse-linked-list/
//用rust来写链表很麻烦，首先是None的问题，其次是used/borrowed value的问题

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
        else if head.clone().unwrap().next.is_none(){return head.clone();}
        let mut prev: Option<Box<ListNode>> = None;
        let mut curr: Option<Box<ListNode>> = head;
        while !curr.is_none() {
            let mut tmp = curr.unwrap();
            curr = tmp.next;
            tmp.next = prev;
            prev = Some(tmp);
        }
  return prev
    }
}