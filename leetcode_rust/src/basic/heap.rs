//! BinaryHeap 模板:默认最大堆;包一层 Reverse 变最小堆
//! Top-K 问题:小根堆维护 K 个最大元素,大根堆维护 K 个最小元素

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// 基础操作:最大堆 / 最小堆
fn heap_ops() -> i32 {
    let mut max_h = BinaryHeap::new();
    max_h.push(1);
    max_h.push(3);
    max_h.push(2);
    let _top = *max_h.peek().unwrap(); // 看堆顶

    let mut min_h = BinaryHeap::new();
    min_h.push(Reverse(3));
    min_h.push(Reverse(1));
    min_h.push(Reverse(2));
    min_h.pop().unwrap().0 // 最小堆堆顶是 1
}

/// Top-K 模板:堆里始终只保留 K 个最大的,堆顶就是第 K 大
fn top_k(nums: &[i32], k: usize) -> Vec<i32> {
    let mut h: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    for &x in nums {
        h.push(Reverse(x));
        if h.len() > k {
            h.pop();
        }
    }
    h.into_iter().map(|Reverse(x)| x).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heap_ops_works() {
        assert_eq!(heap_ops(), 1);
    }

    #[test]
    fn top_k_works() {
        let mut v = top_k(&[3, 1, 5, 2, 4], 3);
        v.sort_unstable(); // 堆内顺序不保证
        assert_eq!(v, vec![3, 4, 5]);
    }
}
