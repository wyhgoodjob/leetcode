//! 栈/队列模板:Vec 当栈,VecDeque 当队列或双端队列,单调栈
//! 括号匹配、逆波兰式用栈;BFS、层序用队列

use std::collections::VecDeque;

/// 栈:push/pop/peek 均为 O(1)
fn stack_ops() -> i32 {
    let mut st: Vec<i32> = Vec::new();
    st.push(1);
    st.push(2);
    st.pop().unwrap(); // 出栈
    *st.last().unwrap() // 栈顶,不弹出
}

/// 队列:push_back 入队,pop_front 出队;双端操作 push_front/pop_back 同样 O(1)
fn queue_ops() -> i32 {
    let mut q: VecDeque<i32> = VecDeque::new();
    q.push_back(1);
    q.push_back(2);
    let x = q.pop_front().unwrap();
    let _ = q.front(); // 队头,不弹出
    x
}

/// 单调栈模板:每个元素右侧第一个比它大的元素
/// 栈内存下标,值保持递减;新元素比栈顶大时弹栈,栈顶的答案就是它
fn next_greater(nums: &[i32]) -> Vec<i32> {
    let mut ans = vec![-1; nums.len()];
    let mut st: Vec<usize> = Vec::new();
    for (i, &x) in nums.iter().enumerate() {
        while let Some(&j) = st.last() {
            if nums[j] >= x {
                break;
            }
            ans[j] = x;
            st.pop();
        }
        st.push(i);
    }
    ans
}

/// 括号匹配模板
fn is_valid(s: &str) -> bool {
    let mut st = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => st.push(c),
            ')' => {
                if st.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if st.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if st.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }
    }
    st.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_ops_works() {
        assert_eq!(stack_ops(), 1);
    }

    #[test]
    fn queue_ops_works() {
        assert_eq!(queue_ops(), 1);
    }

    #[test]
    fn next_greater_works() {
        assert_eq!(next_greater(&[2, 1, 4, 3]), vec![4, 4, -1, -1]);
    }

    #[test]
    fn is_valid_works() {
        assert!(is_valid("()[]{}"));
        assert!(!is_valid("(]"));
    }
}
