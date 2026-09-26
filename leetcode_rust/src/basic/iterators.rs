//! 迭代器模板:Python 列表推导式 -> Rust 的 map/filter/fold 链
//! 通用套路:iter()/into_iter()/iter_mut() -> 链式操作 -> collect() 收尾
//! 闭包参数注意解引用模式:map(|&x| ...) 表示自动解引用

use std::collections::HashMap;

/// map + filter + collect:等价于 [x*2 for x in nums if x*2 > 3]
fn map_filter(nums: &[i32]) -> Vec<i32> {
    nums.iter().map(|&x| x * 2).filter(|&x| x > 3).collect()
}

/// fold:等价于 reduce,常见于求和/聚合
fn fold(nums: &[i32]) -> i32 {
    nums.iter().fold(0, |acc, &x| acc + x)
}

/// enumerate:带下标
fn enumerate(nums: &[i32]) -> Vec<(usize, i32)> {
    nums.iter().enumerate().map(|(i, &x)| (i, x)).collect()
}

/// zip:并行遍历两个序列
fn zip(a: &[i32], b: &[i32]) -> Vec<i32> {
    a.iter().zip(b.iter()).map(|(&x, &y)| x + y).collect()
}

/// any / all / find:存在性判断
fn any_all(nums: &[i32]) -> (bool, bool, Option<&i32>) {
    (
        nums.iter().any(|&x| x > 10),
        nums.iter().all(|&x| x > 0),
        nums.iter().find(|&&x| x == 3),
    )
}

/// fold 计数:一行写出频率表
fn counter(nums: &[i32]) -> HashMap<i32, usize> {
    nums.iter().fold(HashMap::new(), |mut m, &x| {
        *m.entry(x).or_insert(0) += 1;
        m
    })
}

/// 排序 + 取前 N 个:rev 反转 + take 截断
fn top_n(mut v: Vec<i32>, n: usize) -> Vec<i32> {
    v.sort_unstable();
    v.into_iter().rev().take(n).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_filter_works() {
        assert_eq!(map_filter(&[1, 2, 3, 4]), vec![4, 6, 8]);
    }

    #[test]
    fn fold_works() {
        assert_eq!(fold(&[1, 2, 3, 4]), 10);
    }

    #[test]
    fn enumerate_works() {
        assert_eq!(enumerate(&[7, 8]), vec![(0, 7), (1, 8)]);
    }

    #[test]
    fn zip_works() {
        assert_eq!(zip(&[1, 2], &[3, 4]), vec![4, 6]);
    }

    #[test]
    fn any_all_works() {
        let (any, all, found) = any_all(&[1, 2, 3]);
        assert!(!any);
        assert!(all);
        assert_eq!(found, Some(&3));
    }

    #[test]
    fn counter_works() {
        assert_eq!(counter(&[1, 1, 2])[&1], 2);
    }

    #[test]
    fn top_n_works() {
        assert_eq!(top_n(vec![3, 1, 4, 2], 2), vec![4, 3]);
    }
}
