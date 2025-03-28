// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.


// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
            let mut re = first.to_uppercase().collect::<String>();
            re.push_str(c.as_str());
            re
        }
    }

    // match c.next() {
    //     Some(first) => {
    //         first.to_uppercase()
    //         .chain(c)
    //         .collect()
    //     },
    //     None => {String::new()},
    // }

    // match c.next() {
    //     Some(first) => {
    //         format!("{}{}",
    //         first.to_uppercase().collect::<String>(),
    //         c.as_str())
    //     },
    //     None => {String::new()},
    // }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // let mut data = words.into_iter();
    // let mut new_data = vec![];
    // while let Some(d) = data.next() {
    //     let re = capitalize_first(*d);
    //     new_data.push(re);
    // }
    // new_data

    //2
    // let mut new_data = vec![];
    // let mut data = words.into_iter();
    // data.for_each(|s|
    //     new_data.push(capitalize_first(s))
    // );
    // new_data

    ///3
    // let mut new_data = Vec::with_capacity(words.len());
    // for &ele in words {
    //     new_data.push(capitalize_first(ele));
    // }
    // new_data

    //4
    words
        .iter()// 转换为迭代器
        .map(|&s| capitalize_first(s))// 模式匹配直接解引用
        .collect()      

}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
//     let mut data = words.into_iter();
//     let mut result =  String::with_capacity(words.len());
//     while let Some(&d) = data.next() {
//         let re = capitalize_first(d);
//         result.push_str(&re);
//     }
//    result

    ///2
    words.iter()
    .map(|&s| capitalize_first(s))     // 先 map 转换类型
    .fold(String::new(), |mut acc, s| {
        acc.push_str(&s);              // 无中间字符串分配
        acc
    }) 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
