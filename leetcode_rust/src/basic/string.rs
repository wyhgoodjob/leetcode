//! String 模板:&str 与 String、字节/字符处理、构建、切片、分割、KMP
//! 关键点:String 没有下标,不能 s[i];需要随机访问就转 Vec<char> 或 as_bytes()
//! 切片 &s[a..b] 必须落在字符边界,否则 panic

/// 构建:反复拼接用 push_str,格式化用 format!
fn build() -> String {
    let mut s = String::new();
    s.push('a'); // 单字符
    s.push_str("bc"); // 追加字符串
    s += "d"; // 等价 push_str
    format!("{}-{}", s, 1)
}

/// 字节 vs 字符:纯 ASCII(数字/字母)用 as_bytes(),处理 Unicode 用 chars()
fn bytes_and_chars(s: &str) -> usize {
    let bytes = s.as_bytes(); // &[u8],可随机访问
    let chars: Vec<char> = s.chars().collect(); // 需要下标访问时转 Vec<char>
    let _ = (bytes[0], chars[0]);
    s.chars().count()
}

/// 数字字符 -> 数值;反方向:(x as u8 + b'0') as char
fn digit(c: u8) -> i32 {
    (c - b'0') as i32
}

/// 子串切片:边界必须是字符边界
fn slice(s: &str) -> &str {
    &s[1..3]
}

/// 分割;split_whitespace() 可同时按空格/制表符/换行分割
fn split(s: &str) -> Vec<&str> {
    s.split(' ').collect()
}

/// KMP 模板:文本串中找模式串首次出现位置,O(n+m)
fn kmp_search(text: &str, pattern: &str) -> Option<usize> {
    if pattern.is_empty() {
        return Some(0);
    }
    let t = text.as_bytes();
    let p = pattern.as_bytes();

    // next[i]: 模式串前缀 p[..=i] 的最长相等前后缀长度
    let mut next = vec![0usize; p.len()];
    let mut j = 0;
    for i in 1..p.len() {
        while j > 0 && p[i] != p[j] {
            j = next[j - 1];
        }
        if p[i] == p[j] {
            j += 1;
        }
        next[i] = j;
    }

    // 匹配阶段,j 是已匹配长度,失配时按 next 回退
    let mut j = 0;
    for (i, &c) in t.iter().enumerate() {
        while j > 0 && c != p[j] {
            j = next[j - 1];
        }
        if c == p[j] {
            j += 1;
        }
        if j == p.len() {
            return Some(i + 1 - p.len());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kmp() {
        let mother = String::from("test123");
        let child = String::from("st1");
        assert_eq!(kmp_search(&mother, &child), Some(2));
        assert_eq!(kmp_search("abababab", "ababab"), Some(0));
        assert_eq!(kmp_search("hello", "ll"), Some(2));
        assert_eq!(kmp_search("abc", "d"), None);
    }

    #[test]
    fn build_works() {
        assert_eq!(build(), "abcd-1");
    }

    #[test]
    fn bytes_and_chars_works() {
        assert_eq!(bytes_and_chars("ab中"), 3);
    }

    #[test]
    fn digit_works() {
        assert_eq!(digit(b'7'), 7);
    }

    #[test]
    fn slice_works() {
        assert_eq!(slice("abcdef"), "bc");
    }

    #[test]
    fn split_works() {
        assert_eq!(split("a b c"), vec!["a", "b", "c"]);
    }
}
