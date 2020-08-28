// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(-1);
        let mut l1 = head;
        let mut tail = &mut dummy;
        while let Some(mut l1_node) = l1 {
            l1 = l1_node.next.take();
            if let Some(mut l2_node) = l1 {
                l1 = l2_node.next.take();
                l2_node.next = Some(l1_node);
                tail.next = Some(l2_node);
                tail = tail.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                tail.next = Some(l1_node);
            }
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn even_nodes() {
        let n4 = Box::new(ListNode::new(4));
        let mut n3 = Box::new(ListNode::new(3));
        n3.next = Some(n4);
        let mut n2 = Box::new(ListNode::new(2));
        n2.next = Some(n3);
        let mut n1 = Box::new(ListNode::new(1));
        n1.next = Some(n2);

        let nn4 = Box::new(ListNode::new(3));
        let mut nn3 = Box::new(ListNode::new(4));
        nn3.next = Some(nn4);
        let mut nn2 = Box::new(ListNode::new(1));
        nn2.next = Some(nn3);
        let mut nn1 = Box::new(ListNode::new(2));
        nn1.next = Some(nn2);

        let result = Solution::swap_pairs(Some(n1));
        assert_eq!(result, Some(nn1));
    }

    #[test]
    fn odd_nodes() {
        let n5 = Box::new(ListNode::new(5));
        let mut n4 = Box::new(ListNode::new(4));
        n4.next = Some(n5);
        let mut n3 = Box::new(ListNode::new(3));
        n3.next = Some(n4);
        let mut n2 = Box::new(ListNode::new(2));
        n2.next = Some(n3);
        let mut n1 = Box::new(ListNode::new(1));
        n1.next = Some(n2);

        let nn5 = Box::new(ListNode::new(5));
        let mut nn4 = Box::new(ListNode::new(3));
        nn4.next = Some(nn5);
        let mut nn3 = Box::new(ListNode::new(4));
        nn3.next = Some(nn4);
        let mut nn2 = Box::new(ListNode::new(1));
        nn2.next = Some(nn3);
        let mut nn1 = Box::new(ListNode::new(2));
        nn1.next = Some(nn2);

        let result = Solution::swap_pairs(Some(n1));
        assert_eq!(result, Some(nn1));
    }

    #[test]
    fn null_node() {
        let result = Solution::swap_pairs(None);
        assert_eq!(result, None);
    }
}