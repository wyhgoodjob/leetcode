//! HashMap/HashSet 模板:计数器、查找、分组、集合运算
//! 核心:entry API 一次操作完成"存在则更新,不存在则插入",避免查+插两步

use std::collections::{HashMap, HashSet};

/// 计数器(频率统计,热题最常用)
fn counter(nums: &[i32]) -> HashMap<i32, usize> {
    let mut m = HashMap::new();
    for &x in nums {
        *m.entry(x).or_insert(0) += 1;
    }
    m
}

/// 存在则更新,不存在则插入(如滑动窗口内字符计数)
fn update(m: &mut HashMap<i32, i32>, key: i32, delta: i32) {
    m.entry(key).and_modify(|v| *v += delta).or_insert(delta);
}

/// 两数之和模板:边遍历边查已见过的数
fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut seen = HashMap::new();
    for (i, &x) in nums.iter().enumerate() {
        if let Some(&j) = seen.get(&(target - x)) {
            return Some((j, i));
        }
        seen.insert(x, i);
    }
    None
}

/// 分组模板:按 key 聚合列表(字母异位词分组)
fn group<'a>(words: &[&'a str]) -> HashMap<String, Vec<&'a str>> {
    let mut m: HashMap<String, Vec<&str>> = HashMap::new();
    for &w in words {
        let mut chars: Vec<char> = w.chars().collect();
        chars.sort_unstable();
        let key: String = chars.into_iter().collect();
        m.entry(key).or_default().push(w); // or_default:空 Vec 起步
    }
    m
}

/// HashSet:去重/判存在;集合运算 intersection/union/difference
fn set_intersection(a: &[i32], b: &[i32]) -> Vec<i32> {
    let sa: HashSet<_> = a.iter().copied().collect();
    let sb: HashSet<_> = b.iter().copied().collect();
    sa.intersection(&sb).copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_works() {
        let m = counter(&[1, 2, 2, 3, 3, 3]);
        assert_eq!(m[&1], 1);
        assert_eq!(m[&2], 2);
        assert_eq!(m[&3], 3);
    }

    #[test]
    fn update_works() {
        let mut m = HashMap::new();
        update(&mut m, 1, 5);
        update(&mut m, 1, 2);
        assert_eq!(m[&1], 7);
    }

    #[test]
    fn two_sum_works() {
        assert_eq!(two_sum(&[2, 7, 11, 15], 9), Some((0, 1)));
        assert_eq!(two_sum(&[3, 2, 4], 6), Some((1, 2)));
        assert_eq!(two_sum(&[1, 2, 3], 100), None);
    }

    #[test]
    fn group_works() {
        let m = group(&["eat", "tea", "tan", "ate"]);
        assert_eq!(m["aet"].len(), 3);
        assert_eq!(m["ant"].len(), 1);
    }

    #[test]
    fn set_intersection_works() {
        let mut r = set_intersection(&[1, 2, 3], &[2, 3, 4]);
        r.sort_unstable(); // HashSet 遍历顺序不定
        assert_eq!(r, vec![2, 3]);
    }
}
