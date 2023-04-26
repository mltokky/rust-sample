/*
 * 構造体を使用して関係のあるデータを構造化する
 * https://doc.rust-jp.rs/book-ja/ch05-02-example-structs.html
 */

struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// derive注釈でDebugトレイトを追加
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// Rectangle構造体上にメソッドを実装する
impl Rectangle {
    // メソッドの第1引数には自身のインスタンスを表す引数を定義する（＝レシーバーを持つ関数）
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// 複数に分けてimplブロックを書くことが可能
impl Rectangle {
    // selfを引数に取らない関数 → 「関連関数」
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// タプル構造体
struct Color(u32, u32, u32);

fn build_user(name: String, email: String) -> User {
    User {
        name: name,
        email: email,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    // 構造体のインスタンスを生成
    let user1 = User {
        name: "test taro".to_string(),
        email: "test@example.com".to_string(),
        active: true,
        sign_in_count: 1,
    };
    println!("user1: {}({})", user1.name, user1.email);

    // インスタンスを生成するメソッドを通して取得
    let user2 = build_user("test jiro".to_string(), "test2@example.com".to_string());
    println!("user2: {}({})", user2.name, user2.email);

    // 構造体更新記法を使ってインスタンスを生成
    let user3 = User {
        name: "test saburo".to_string(),
        email: "test3@example.com".to_string(),
        ..user1 // 残りの値はuser1と同様のものを使用する
    };
    println!(
        "user3: {}({}) -- is_login?: {}, signin count: {}",
        user3.name, user3.email, user3.active, user3.sign_in_count
    );

    // タプル構造体の生成
    let red = Color(255, 0, 0);
    println!("red color: {}, {}, {}", red.0, red.1, red.2);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    // Debugトレイトが実装された構造体の全フィールドを出力する
    println!("rect1: {:?}", rect1);
    println!("rect2: {:?}", rect2);
    println!("rect3: {:?}", rect3);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Rectangleクラスの関連関数 `square()` を呼び出す
    let square = Rectangle::square(22);
    println!("square: {:?}", square);
}
