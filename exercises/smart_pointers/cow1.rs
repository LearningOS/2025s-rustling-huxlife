// cow1.rs
//
// This exercise explores the Cow, or Clone-On-Write type. Cow is a
// clone-on-write smart pointer. It can enclose and provide immutable access to
// borrowed data, and clone the data lazily when mutation or ownership is
// required. The type is designed to work with general borrowed data via the
// Borrow trait.
//
// This exercise is meant to show you what to expect when passing data to Cow.
// Fix the unit tests by checking for Cow::Owned(_) and Cow::Borrowed(_) at the
// TODO markers.
//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.


use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}
/*### **`Cow` 的核心逻辑**
`Cow`（Clone-On-Write）的行为取决于它的初始状态和是否需要修改数据：
1. 如果 `Cow` 是 **`Borrowed`**：
   - 数据未被修改时，仍然是 `Borrowed`。
   - 数据被修改时，会克隆数据并变为 `Owned`。
2. 如果 `Cow` 是 **`Owned`**：
   - 无论数据是否被修改，都会保持 `Owned` 状态。*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // No clone occurs because `input` doesn't need to be mutated.
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Borrowed(_) => Ok(()), // 数据未被修改，仍然是 Borrowed
            _ => Err("Expected borrowed value"),
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // We can also pass `slice` without `&` so Cow owns it directly. In this
        // case no mutation occurs and thus also no clone, but the result is
        // still owned because it was never borrowed or mutated.
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()), // 数据本来就是 Owned，未被修改
            _ => Err("Expected owned value"),
        }
    }

    /* #### **逻辑**
1. `Cow::from(slice)`：
   - 这里传入的是一个 `Vec`，`Cow` 会直接将其包装为 `Owned`。
   - `Cow` 的初始状态是 `Owned`。

2. 调用 `abs_all(&mut input)`：
   - `abs_all` 遍历数据，发现 `-1` 是负数，因此需要修改数据。
   - 调用 `to_mut()` 时，`Cow` 已经是 `Owned`，所以直接修改底层数据，而不会触发克隆。

3. **为什么返回 `Owned`？**
   - 数据已经是 `Owned`，并且被修改。
   - `Cow` 的状态仍然是 `Owned`，因为它一开始就是 `Owned`。*/
    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // Of course this is also the case if a mutation does occur. In this
        // case the call to `to_mut()` returns a reference to the same data as
        // before.
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }
}
/* 逐行解释
reference_mutation 测试
创建一个借用的 Cow，引用 slice。
因为 slice 包含负数，调用 abs_all 时会触发 to_mut，导致数据被克隆并变为 Owned。
检查 Cow 是否变为 Owned，因为数据被修改。
reference_no_mutation 测试
;
创建一个借用的 Cow，引用 slice。
因为 slice 中没有负数，调用 abs_all 时不会触发 to_mut，数据仍然是 Borrowed。
}
检查 Cow 是否仍然是 Borrowed，因为数据未被修改。
owned_no_mutation 测试
;
创建一个拥有的 Cow，直接拥有 slice 的数据。
因为数据未被修改，Cow 仍然是 Owned。
}
检查 Cow 是否仍然是 Owned，因为数据未被修改。
owned_mutation 测试
;
创建一个拥有的 Cow，直接拥有 slice 的数据。
因为数据包含负数，调用 abs_all 时会修改数据，但不会触发克隆，因为数据已经是 Owned。
检查 Cow 是否仍然是 Owned，因为数据已经是 Owned，并且被修改。*/