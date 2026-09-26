//! 二叉树模板:LeetCode 定义 Option<Rc<RefCell<TreeNode>>>
//! 递归(DFS)为主:borrow() 只读、borrow_mut() 修改;层序(BFS)用 VecDeque

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

type Tree = Option<Rc<RefCell<TreeNode>>>;

/// 前序遍历(递归模板):中序/后序调换三行顺序即可
fn preorder(root: &Tree, out: &mut Vec<i32>) {
    if let Some(node) = root {
        let n = node.borrow();
        out.push(n.val);
        preorder(&n.left, out);
        preorder(&n.right, out);
    }
}

/// 递归返回值模板:最大深度
fn max_depth(root: &Tree) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let n = node.borrow();
            1 + max_depth(&n.left).max(max_depth(&n.right))
        }
    }
}

/// 层序遍历(BFS):队列逐层处理
fn level_order(root: &Tree) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    if let Some(r) = root {
        q.push_back(r.clone()); // clone 只增加 Rc 引用计数,不复制数据
    }
    while !q.is_empty() {
        let mut level = Vec::new();
        for _ in 0..q.len() {
            let node = q.pop_front().unwrap();
            let n = node.borrow();
            level.push(n.val);
            if let Some(l) = &n.left {
                q.push_back(l.clone());
            }
            if let Some(r) = &n.right {
                q.push_back(r.clone());
            }
        }
        res.push(level);
    }
    res
}

/// 由层序数组建树(测试辅助,None 表示空节点;按完全二叉树下标定位,测试够用)
fn build(vals: &[Option<i32>]) -> Tree {
    if vals.is_empty() {
        return None;
    }
    let nodes: Vec<Tree> = vals
        .iter()
        .map(|v| v.map(|x| Rc::new(RefCell::new(TreeNode::new(x)))))
        .collect();
    for (i, node) in nodes.iter().enumerate() {
        if let Some(n) = node {
            let mut n = n.borrow_mut();
            let l = 2 * i + 1;
            let r = 2 * i + 2;
            if l < nodes.len() {
                n.left = nodes[l].clone();
            }
            if r < nodes.len() {
                n.right = nodes[r].clone();
            }
        }
    }
    nodes[0].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tree() -> Tree {
        build(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)])
    }

    #[test]
    fn preorder_works() {
        let mut out = Vec::new();
        preorder(&tree(), &mut out);
        assert_eq!(out, vec![3, 9, 20, 15, 7]);
    }

    #[test]
    fn max_depth_works() {
        assert_eq!(max_depth(&tree()), 3);
        assert_eq!(max_depth(&None), 0);
    }

    #[test]
    fn level_order_works() {
        assert_eq!(level_order(&tree()), vec![vec![3], vec![9, 20], vec![15, 7]]);
    }
}
