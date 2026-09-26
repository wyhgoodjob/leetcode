pub struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut ans = f64::NEG_INFINITY;
        let mut cur = 0;
        for i in 0..nums.len() {
            cur += nums[i];
            if i + 1 < k {
                continue;
            }
            let left = i + 1 - k;
            ans = ans.max(cur as f64 / k as f64);
            cur -= nums[left];
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(got: f64, expected: f64) {
        assert!(
            (got - expected).abs() < 1e-5,
            "expected {expected}, got {got}"
        );
    }

    #[test]
    fn example_1() {
        assert_close(Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4), 12.75);
    }

    #[test]
    fn example_2() {
        assert_close(Solution::find_max_average(vec![5], 1), 5.0);
    }

    #[test]
    fn all_negative() {
        assert_close(Solution::find_max_average(vec![-3, -1, -4, -2], 2), -2.0);
        assert_close(Solution::find_max_average(vec![-100], 1), -100.0);
    }

    #[test]
    fn window_equals_whole_array() {
        assert_close(Solution::find_max_average(vec![1, 2, 3, 4], 4), 2.5);
        assert_close(Solution::find_max_average(vec![4, 3, 2, 1], 4), 2.5);
    }

    #[test]
    fn best_window_at_the_start() {
        assert_close(Solution::find_max_average(vec![10, 10, 1, 1, 1], 2), 10.0);
    }

    #[test]
    fn best_window_at_the_end() {
        assert_close(Solution::find_max_average(vec![1, 1, 1, 10, 10], 2), 10.0);
    }

    #[test]
    fn single_element_windows() {
        assert_close(Solution::find_max_average(vec![-1, 4, 2, -3], 1), 4.0);
    }

    #[test]
    fn repeated_values() {
        assert_close(Solution::find_max_average(vec![7, 7, 7, 7, 7], 3), 7.0);
    }

    #[test]
    fn fractional_result() {
        assert_close(Solution::find_max_average(vec![1, 2], 2), 1.5);
    }
}
