// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    // 提供一个公共函数来调用宏
    pub fn call_macro() {
        my_macro!();
    }
}

fn main() {
    // 通过模块的公共函数间接调用宏
    macros::call_macro();
}
