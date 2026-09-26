//! 链表模板:LeetCode 节点定义是 Option<Box<ListNode>>(Box 独占所有权)
//! 关键技巧:① dummy 头节点省去删头特判 ② 需要"拆"节点时用 take() 避开借用冲突

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

/// 由数组建链表(测试辅助)
fn from_slice(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut cur = &mut dummy;
    for &v in vals {
        cur.next = Some(Box::new(ListNode::new(v)));
        cur = cur.next.as_mut().unwrap();
    }
    dummy.next
}

/// 遍历:cur 一路向后走
fn to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut res = Vec::new();
    let mut cur = head;
    while let Some(node) = cur {
        res.push(node.val);
        cur = &node.next;
    }
    res
}

/// 反转链表:核心模板,head 与 prev 交替前进
fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    while let Some(mut node) = head {
        head = node.next.take(); // take:把 next 所有权拆出来,node 不再被它借用
        node.next = prev;
        prev = Some(node);
    }
    prev
}

/// 快慢指针:找中间节点(偶数长度返回靠右的那个)
fn middle(head: &Option<Box<ListNode>>) -> Option<&Box<ListNode>> {
    let mut slow = head;
    let mut fast = head;
    while let Some(node) = fast {
        if node.next.is_none() {
            break;
        }
        fast = &node.next.as_ref().unwrap().next;
        slow = &slow.as_ref().unwrap().next;
    }
    slow.as_ref()
}

/// 删除第 k 个节点(1-based):dummy 头节点模板,删头节点无需特判
fn remove_kth(mut head: Option<Box<ListNode>>, k: usize) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode::new(0)));
    dummy.as_mut().unwrap().next = head.take();
    let mut cur = &mut dummy;
    for _ in 1..k {
        cur = &mut cur.as_mut().unwrap().next;
    }
    // cur 指向待删节点的前一个,用 take 摘除
    let node = cur.as_mut().unwrap();
    node.next = node.next.take().unwrap().next;
    dummy.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_slice_works() {
        assert_eq!(to_vec(&from_slice(&[1, 2, 3])), vec![1, 2, 3]);
        assert!(from_slice(&[]).is_none());
    }

    #[test]
    fn reverse_works() {
        assert_eq!(
            to_vec(&reverse(from_slice(&[1, 2, 3, 4]))),
            vec![4, 3, 2, 1]
        );
    }

    #[test]
    fn middle_works() {
        let head = from_slice(&[1, 2, 3, 4, 5]);
        assert_eq!(middle(&head).map(|n| n.val), Some(3));
        let head2 = from_slice(&[1, 2, 3, 4]);
        assert_eq!(middle(&head2).map(|n| n.val), Some(3)); // 偶数取右中位
    }

    #[test]
    fn remove_kth_works() {
        assert_eq!(
            to_vec(&remove_kth(from_slice(&[1, 2, 3, 4]), 2)),
            vec![1, 3, 4]
        );
        assert_eq!(
            to_vec(&remove_kth(from_slice(&[1, 2, 3]), 1)),
            vec![2, 3]
        ); // 删头节点
    }
}
