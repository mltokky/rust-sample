/*
 * Enumとパターンマッチング
 * https://doc.rust-jp.rs/book-ja/ch06-00-enums.html
 */

use rand::random;

// 列挙体
#[derive(Debug)]
enum Fruits {
    Apple(String),            // 列挙子にString型のデータを紐付ける
    Banana(String, String),   // 2つ以上指定も可能
    Cherry { color: String }, // 匿名構造体を含む
    Durian,
    Fig,
    Grape,
}

// 列挙体もimplによるメソッドの実装が可能
impl Fruits {
    fn call(&self) {
        println!("call -> {:?}", self);
    }
}

enum Coin {
    One,
    Five,
    Ten,
    Fifty,
    OneHundred,
    FiveHundred,
}

fn main() {
    let apple = Fruits::Apple(String::from("red")); // 呼び出し時に文字列を指定する
    println!("fruit: {:?}", apple);
    let banana = Fruits::Banana(String::from("yellow"), String::from("Japan"));
    println!("fruit: {:?}", banana);
    let cherry = Fruits::Cherry {
        color: String::from("red"),
    };
    println!("fruit: {:?}", cherry);
    println!();

    let durian = Fruits::Durian;
    durian.call();

    let coin = Coin::One;
    let coin_number = get_coin_number(Some(coin));
    println!("coin is {}", coin_number);
    let coin_number = get_coin_number(None);
    println!("coin is {}", coin_number);
}

// match式を使う → その値を返す
fn get_coin_number(coin: Option<Coin>) -> u32 {
    if let Some(val) = coin {
        match val {
            Coin::One => {
                // 複数行になる場合は{}で囲む
                println!("min coin!!");
                1
            }
            Coin::Five => 5,
            Coin::Ten => 10,
            Coin::Fifty => 50,
            Coin::OneHundred => 100,
            Coin::FiveHundred => {
                println!("max coin!!!");
                500
            }
        }
    } else {
        println!("coin is nothing!!!!");
        0
    }
}

fn get_random_value() -> u8 {
    return random();
}
