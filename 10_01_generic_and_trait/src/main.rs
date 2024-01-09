fn main() {
    println!("Hello, world!");

    let list = vec![34, 50, 25, 100, 65];
    // let result = largest_i32(&list);
    let result = largest(&list);  // ジェネリック化したメソッドに置き換え
    println!("largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest_char(&char_list);
    let result = largest(&char_list);  // ジェネリック化したメソッドに置き換え
    println!("largest char is {}", result);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("longest string is {}", result);
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

// 上記2つのメソッドをジェネリクスを使って汎用化
// → 比較可能かどうかは `PartialOrd` トレイトが条件
// → 共有されたリストから値をムーブする場合は `Copy` トレイトが必要
// 2つのトレイトを満たす必要がある場合は `+` を使って両方を記載する
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

// `Copy` トレイトを不要にする方法
fn largest2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

// ライフタイム注釈を付け、s1, s2と戻り値が同じライフタイムであることをコンパイラに知らせる
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() < s2.len() {
        s2
    } else {
        s1
    }
}