/*
 * 一般的なプログラミングの概念
 * https://doc.rust-jp.rs/book-ja/ch03-00-common-programming-concepts.html
 */

use std::io;

fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    let x = 0x11;
    println!("0x11: {}", x);
    let x = 100_000;
    println!("100_000: {}", x);
    let x = 0o21;
    println!("0o21: {}", x);
    let x = 0b1101_0001;
    println!("0b1101_0001: {}", x);

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    // タプル(tuple)
    let tup = (500, 6.4, 1, "str");
    println!("tuple value: {}", tup.3);

    // 配列
    let arr = ["1", "2", "3"];
    println!("string array: {}", arr[1]);
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("int array: {}", arr[4]);

    // 別の関数を呼ぶ
    anothor_function(11);
    let answer = five();
    println!("five: {}", answer);

    let y = {
        let x = 3;
        x * 2  // <-- 式として値を返す場合はセミコロン無し
    };
    println!("y is {}", y);

    // 条件文
    let number = 11;
    if number < 20 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }
    let is_large = if number > 20 { "true" } else { "false" };
    println!("Is the number large? -> {}", is_large);

    // ループ
    println!("----- start infinity loop -----");
    let mut count = 1;
    'counting_up: loop {  // loopにラベルをつける
        println!("count: {}", count);

        println!("input number: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("cannot read input");
        let input_result: i32 = match input.trim().parse() {
            Ok(num) => if num < 0 { continue } else { num },
            Err(_) => continue,
        };

        if input_result == 0 {
            break 'counting_up;
        }
        count += 1;
    }

    // whileループ
    println!("----- start while loop -----");
    let mut counter = 1;
    while counter <= 5 {
        println!("while loop -- {}", counter);
        counter += 1;
    }

    // forループ
    println!("----- start for loop -----");
    let data = [1, 2, 3, 4, 5, 6, 7];
    for element in data {
        println!("element is: {}", element);
    }
    for element in (1..4).rev() {  // <-- (1..4) → Range型
        println!("element is: {}", element);
    }
}

// 引数あり、戻り値なしの関数
fn anothor_function(x: i32) {
    println!("this is anothor function: {}", x);
}

// 引数なし、戻り値ありの関数
fn five() -> i32 {
    22  // <-- 戻り値を返す際はセミコロンなし
}