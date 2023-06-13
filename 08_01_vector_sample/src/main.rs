/*
 * 8.1. ベクタで値のリストを保持する
 * https://doc.rust-jp.rs/book-ja/ch08-01-vectors.html
 */

fn main() {
    // 新しいベクタの作成 → 型はジェネリクスで指定する
    let mut v1: Vec<i32> = Vec::new();
    // 初期値を指定したベクタの作成
    let mut v2 = vec![1, 2, 3, 4, 5];

    // ベクタへの値の追加：push()
    v1.push(6);
    v1.push(7);
    v1.push(8);
    v1.push(9);
    v1.push(10);
    v2.push(6);
    v2.push(7);
    v2.push(8);
    v2.push(9);
    v2.push(10);

    // ベクタの値を取得 Part.1
    let third: &i32 = &v1[2];
    println!("<v1> value is {}", third);
    // 以下はアクセスするとpanicになるので実行しないこと
    // let does_not_exist = &v1[100];

    // ベクタの値を取得 Part.2
    match v2.get(2) {
        Some(third) => println!("value is {}", third),
        None => println!("value is no el ment"),
    }

    // ベクタ内のすべての値にアクセスするにはfor文を使う
    println!("--- v1 loop ---");
    for i in &v1 {
        println!("value: {}", i);
    }
    println!("--- v2 loop ---");
    for i in &v2 {
        println!("value: {}", i);
    }

    // 異なる方を1つのベクタにまとめたい時は列挙体を使う
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadSheetCell::Int(192),
        SpreadSheetCell::Float(0.14225),
        SpreadSheetCell::Text(String::from("これはテキスト")),
    ];
    for cell in &row {
        match &cell {
            SpreadSheetCell::Int(val) => println!("int value: {}", val),
            SpreadSheetCell::Float(val) => println!("float value: {}", val),
            SpreadSheetCell::Text(val) => println!("text value: {}", val),
        }
    }
}
