pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::<i32>::new();
        for s in &tokens {
            match s.as_str() {
                "+" => {
                    let b = stack.pop().expect("No element!");
                    let a = stack.pop().expect("No element!");
                    stack.push(a+b);
                }
                "-" => {
                    let b = stack.pop().expect("No element!");
                    let a = stack.pop().expect("No element!");
                    stack.push(a-b);
                }
                "*" => {
                    let b = stack.pop().expect("No element!");
                    let a = stack.pop().expect("No element!");
                    stack.push(a*b);
                }
                "/" => {
                    let b = stack.pop().expect("No element!");
                    let a = stack.pop().expect("No element!");
                    stack.push(a/b);
                }
                _ => {
                    let num = s.parse::<i32>().expect("Not a Number!");
                    stack.push(num);
                }
            }
        }
        return stack.pop().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokens(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(Solution::eval_rpn(tokens(&["2", "1", "+", "3", "*"])), 9);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::eval_rpn(tokens(&["4", "13", "5", "/", "+"])), 6);
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::eval_rpn(tokens(&[
                "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"
            ])),
            22
        );
    }

    #[test]
    fn single_operand() {
        assert_eq!(Solution::eval_rpn(tokens(&["18"])), 18);
    }

    #[test]
    fn multiple_operations() {
        assert_eq!(Solution::eval_rpn(tokens(&["5", "1", "2", "+", "4", "*", "+", "3", "-"])), 14);
    }

    #[test]
    fn negative_operands() {
        assert_eq!(Solution::eval_rpn(tokens(&["-1", "1", "-"])), -2);
        assert_eq!(Solution::eval_rpn(tokens(&["3", "-2", "*"])), -6);
    }

    #[test]
    fn division_truncates_toward_zero() {
        assert_eq!(Solution::eval_rpn(tokens(&["7", "2", "/"])), 3);
        assert_eq!(Solution::eval_rpn(tokens(&["-7", "2", "/"])), -3);
    }
}