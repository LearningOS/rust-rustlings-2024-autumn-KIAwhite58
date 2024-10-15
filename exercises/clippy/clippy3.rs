// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        println!("my_option is none");
    } else {
        // 处理 Some 的情况
        // my_option.unwrap(); // 这里不需要解包，因为我们已经知道它不是 None
    }

    let my_arr = [
        [-1, -2, -3],
        [-4, -5, -6],
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear(); // 清空向量
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 正确交换两个变量的值
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
