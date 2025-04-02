// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}


fn main() {
    my_macro!();
}
/*2. 在外部文件中定义宏并导入
如果宏定义在另一个文件中，可以通过 #[macro_use] 将其导入。

示例代码：
macros.rs 文件：
#[macro_export]
macro_rules! my_macro {
    () => {
        println!("This is my macro from another file!");
    };
}
m
主文件：
#[macro_use]
mod macros;

fn main() {
    my_macro!(); // 使用导入的宏
}

#[macro_export] 的作用：
将宏导出，使其可以被其他模块或文件使用。
#[macro_use] 的作用：
将 macros 模块中的宏导入到当前作用域。 */