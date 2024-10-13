// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn longest(x: String, y: String) -> String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let mut result = String::new(); // 使用 mut 关键词来允许修改 result
    {
        let string2 = String::from("xyz");
        // 这里我们可以直接传递 string1 和 string2，因为 longest 函数现在接受 String 类型
        result = longest(string1.clone(), string2);
    }
    // 由于 result 是 String 类型，我们可以直接打印它
    println!("The longest string is '{}'", result);
}
