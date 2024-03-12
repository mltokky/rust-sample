use List::{Cons, Nil};

fn main() {
    // ヒープ領域を確保して値を保存・表示
    let b = Box::new(5);
    println!("Box's value: {}", b);

    // 入れ子で同じ型を持つ例
    println!("----- List -----");
    let mut list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let mut is_continue = true;
    while is_continue {
        is_continue = match list {
            Cons(value, next) => {
                list = *next;
                println!("value: {}", value);
                true
            },
            Nil => {
                println!("End");
                false
            }
        }
    }
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}