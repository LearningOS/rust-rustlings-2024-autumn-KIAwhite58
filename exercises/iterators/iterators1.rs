// iterators1.rs
//
// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
//
// Make me compile by filling in the `???`s
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    // Step 1: 创建一个迭代器来遍历 my_fav_fruits 中的元素
    let mut my_iterable_fav_fruits = my_fav_fruits.iter();   

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    // Step 2: 断言下一个元素是 "custard apple"
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));     
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    // Step 3: 断言下一个元素是 "peach"
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));     
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    // Step 4: 断言没有更多的元素，因此 next() 返回 None
    assert_eq!(my_iterable_fav_fruits.next(), None);
}
