// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

// In an imperative language, you might write a for loop that updates
// a mutable variable. Or, you might write code utilizing recursion
// and a match clause. In Rust you can take another functional
// approach, computing the factorial elegantly with ranges and iterators.


pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    //     1..=num

    // 创建一个包含范围 [1, num] 的迭代器
    // = 表示包含上边界
    // 当 num = 0 时，范围为空
    // fold(1, |acc, x| acc * x)

    // fold 是一个迭代器方法，用于累积计算
    // 第一个参数 1 是初始值
    // |acc, x| acc * x 是累积函数
    // acc 是累积值
    // x 是当前数字
    // 返回它们的乘积
    (1..=num).fold(1, |acc, x| acc * x)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
