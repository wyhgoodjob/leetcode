pub struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let k = k as usize;
        let mut ans = 0;
        let mut vowel = 0;
        let is_vowel = |c: u8| -> bool { matches!(c, b'a' | b'e' | b'i' | b'o' | b'u') };

        for (i, &c) in s.iter().enumerate() {
            if is_vowel(c) {
                vowel += 1;
            }
            if i + 1 < k {
                continue;
            }
            ans = ans.max(vowel);
            let left = i + 1 - k;
            let out = s[left];
            if is_vowel(out) {
                vowel -= 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn max_vowels(s: &str, k: i32) -> i32 {
        Solution::max_vowels(s.to_string(), k)
    }

    #[test]
    fn example_1() {
        assert_eq!(max_vowels("abciiidef", 3), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(max_vowels("aeiou", 2), 2);
    }

    #[test]
    fn example_3() {
        assert_eq!(max_vowels("leetcode", 3), 2);
    }

    #[test]
    fn window_equals_whole_string() {
        assert_eq!(max_vowels("aeiou", 5), 5);
        assert_eq!(max_vowels("bcdfg", 5), 0);
    }

    #[test]
    fn single_character_window() {
        assert_eq!(max_vowels("abcde", 1), 1);
        assert_eq!(max_vowels("xyz", 1), 0);
    }

    #[test]
    fn all_consonants() {
        assert_eq!(max_vowels("bcdfghjklmnpqrstvwxyz", 4), 0);
    }

    #[test]
    fn all_vowels() {
        assert_eq!(max_vowels("aeiouaeiou", 4), 4);
    }

    #[test]
    fn best_window_at_the_start() {
        assert_eq!(max_vowels("aeiouvwxyz", 3), 3);
    }

    #[test]
    fn best_window_at_the_end() {
        assert_eq!(max_vowels("vwxyzaeiou", 3), 3);
    }

    #[test]
    fn repeated_letters() {
        assert_eq!(max_vowels("aaaaabbbbb", 5), 5);
        assert_eq!(max_vowels("bbbbbaaaaa", 5), 5);
    }

    #[test]
    fn uppercase_is_not_a_vowel() {
        // LeetCode 的输入只包含小写字母，但顺便固定一下大小写敏感的行为。
        assert_eq!(max_vowels("AEIOUaeiou", 5), 5);
    }
}
