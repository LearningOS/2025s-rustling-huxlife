// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.


struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: The `ptr` contains an owned box of `Foo` by contract. We
    // simply reconstruct the box from that pointer.
    let mut ret: Box<Foo> =  unsafe { Box::from_raw(ptr) };
    ret.b = Some("hello".to_owned());
    ret
}

// Box::into_raw(): Box 转换为原始指针
// Box::from_raw(): 原始指针转换回 Box

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        /**
         * 1，创建一个Box
         * 2，获取 data.a 的内存地址，将其转换为常量指针，再转换为 usize 类型
         * 3，通过 raw_pointer_to_box 传递一个 原始指针（由 Box::into_raw(data)生成），最后得到
         * 一个Box 类型
         * 所以 ret 是Box 类型，其实就和data 一样了
         */
        let data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize;
        // SAFETY: We pass an owned box of `Foo`.
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };// 将原始指针恢复为 Box

        let ptr_2 = &ret.a as *const u128 as usize;

        assert!(ptr_1 == ptr_2);
        assert!(ret.b == Some("hello".to_owned()));
    }
}
