//! Vec 模板:初始化、遍历、双指针、滑动窗口、前缀和、二分查找、排序
//! LeetCode 的数组题基本都是 Vec<i32>;下标访问用 &v[..] 切片或直接索引

use std::cmp::Ordering;

/// 初始化方式
fn init() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);

    let _ = vec![0; 10]; // 定长填充
    let _ = (1..=5).collect::<Vec<i32>>(); // 范围
    let _: Vec<i32> = (0..5).map(|x| x * x).collect(); // 由迭代器生成
    let _ = vec![vec![0; 3]; 4]; // 二维矩阵(每个内层 vec 独立)
    let _ = &v[..]; // Vec -> 切片
}

/// 遍历:只读 / 带下标 / 修改(修改必须 iter_mut)
fn iterate(v: &mut [i32]) -> i32 {
    let mut sum = 0;
    for &x in v.iter() {
        sum += x;
    }
    let enumerated: Vec<(usize, i32)> = v.iter().enumerate().map(|(i, &x)| (i, x)).collect();
    let _ = enumerated;
    for x in v.iter_mut() {
        *x *= 2;
    }
    sum
}

/// 双指针模板:左右向中间收敛(两数之和 II、接雨水等)
fn two_pointer(nums: &[i32], target: i32) -> (usize, usize) {
    let (mut l, mut r) = (0, nums.len() - 1);
    while l < r {
        match (nums[l] + nums[r]).cmp(&target) {
            Ordering::Less => l += 1,
            Ordering::Greater => r -= 1,
            Ordering::Equal => return (l, r),
        }
    }
    unreachable!("输入保证有解")
}

/// 滑动窗口模板:和 <= k 的最长子数组
fn sliding_window(nums: &[i32], k: i32) -> usize {
    let (mut l, mut sum, mut ans) = (0, 0, 0);
    for r in 0..nums.len() {
        sum += nums[r];
        while sum > k {
            sum -= nums[l];
            l += 1;
        }
        ans = ans.max(r - l + 1);
    }
    ans
}

/// 前缀和:区间 [l, r] 的和 = pre[r+1] - pre[l]
fn prefix_sum(nums: &[i32]) -> Vec<i32> {
    let mut pre = vec![0; nums.len() + 1];
    for (i, &x) in nums.iter().enumerate() {
        pre[i + 1] = pre[i] + x;
    }
    pre
}

/// 二分查找模板:左闭右开,找第一个 >= target 的位置
/// 变形:找 > target 改 nums[mid] <= target;找 == target 再检查 nums[lo]
fn lower_bound(nums: &[i32], target: i32) -> usize {
    let (mut lo, mut hi) = (0, nums.len());
    while lo < hi {
        let mid = lo + (hi - lo) / 2; // 防溢出
        if nums[mid] < target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

/// 排序 / 去重 / 反转(dedup 只存在于 Vec,故参数用 &mut Vec)
fn sort_utils(v: &mut Vec<i32>) {
    v.sort_unstable(); // 原地排序;需要稳定排序时用 sort()
    v.dedup(); // 去重(要求已排序)
    v.reverse();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_works() {
        init();
    }

    #[test]
    fn iterate_works() {
        let mut v = vec![1, 2, 3];
        let sum = iterate(&mut v);
        assert_eq!(sum, 6);
        assert_eq!(v, vec![2, 4, 6]);
    }

    #[test]
    fn two_pointer_works() {
        assert_eq!(two_pointer(&[2, 7, 11, 15], 9), (0, 1));
        assert_eq!(two_pointer(&[1, 2, 3, 4, 5], 7), (1, 4));
    }

    #[test]
    fn sliding_window_works() {
        assert_eq!(sliding_window(&[2, 3, 1, 2, 4, 3], 5), 2);
    }

    #[test]
    fn prefix_sum_works() {
        assert_eq!(prefix_sum(&[1, 2, 3, 4]), vec![0, 1, 3, 6, 10]);
    }

    #[test]
    fn lower_bound_works() {
        assert_eq!(lower_bound(&[1, 3, 3, 5, 7], 3), 1);
        assert_eq!(lower_bound(&[1, 3, 3, 5, 7], 4), 3);
        assert_eq!(lower_bound(&[1, 3, 3, 5, 7], 8), 5);
    }

    #[test]
    fn sort_utils_works() {
        let mut v = vec![3, 1, 2, 2];
        sort_utils(&mut v);
        assert_eq!(v, vec![3, 2, 1]);
    }
}
