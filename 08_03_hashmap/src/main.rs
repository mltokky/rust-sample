/*
 * 8.3. キーとそれに紐づいた値をハッシュマップに格納する
 * https://doc.rust-jp.rs/book-ja/ch08-03-hash-maps.html
 */

use std::collections::HashMap;

fn main() {
    // HashMapの生成
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // VectorからHashMapを生成
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 20];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // 値を取得
    let key = String::from("Blue");
    println!("team: {}, value: {}", key, scores.get(&key).unwrap());
    for (key, value) in &scores {
        println!("team: {}, value: {}", key, value);
    }
    // 以下でもHashMapの中身を表示可能
    println!("scores: {:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Orange");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_nameとfield_valueは所有権が移るため、使用不可になる（以下はエラー）
    // println!("{} : {}", field_name, field_value);
}
