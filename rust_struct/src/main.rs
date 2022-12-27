/*
 * https://doc.rust-jp.rs/book-ja/ch05-02-example-structs.html
 */

struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
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
}
