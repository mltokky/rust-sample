/*
 * 8.2. 文字列でUTF-8でエンコードされたテキストを保持する
 * https://doc.rust-jp.rs/book-ja/ch08-02-strings.html
 */

fn main() {
    let s = String::from("これは文字列");

    // 文字列スライス → Stringへ変換
    let data = "initial string data";
    let s = data.to_string();

    println!("----- UTF-8 encode -----");
    let hello = String::from("السلام عليكم");
    println!("{}", hello);
    let hello = String::from("Dobrý den");
    println!("{}", hello);
    let hello = String::from("Hello");
    println!("{}", hello);
    let hello = String::from("שָׁלוֹם");
    println!("{}", hello);
    let hello = String::from("नमस्ते");
    println!("{}", hello);
    let hello = String::from("こんにちは");
    println!("{}", hello);
    let hello = String::from("안녕하세요");
    println!("{}", hello);
    let hello = String::from("你好");
    println!("{}", hello);
    let hello = String::from("Olá");
    println!("{}", hello);
    let hello = String::from("Здравствуйте");
    println!("{}", hello);
    let hello = String::from("Hola");
    println!("{}", hello);

    // Stringに新たに文字列を追加
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!("{}", s);
    // strのスライスを渡したので、所有権は奪っていない → 使用可能
    println!("s2 is {}", s2);

    let s3 = String::from("Hello, ");
    let s4 = String::from("Rust world!");
    // ジェネリクスで定義された add() 関数が呼ばれている
    let s5 = s3 + &s4;
    println!("s5: {}", s5);

    // 複雑な文字列の連結にはformat!マクロを使う　所有権も奪わない
    let s6 = String::from("stone");
    let s7 = String::from("sky");
    let s8 = String::from("sea");
    let s9 = format!("{}-{}-{}", s6, s7, s8);
    println!("{}", s9);

    let s10 = "日本語で文字列を打ちます";
    // let s10_slice = &s10[0..4];  // これはエラー：マルチバイト文字の途中の位置（'本'のインデックスが3..6であり、その間までをスライスにしようとしたため）
    // println!("s10 slice: {}", s10_slice);
    // 正確に1文字ずつアクセスするには chars() を使用する
    println!("s10: {}", s10);
    println!("----- chars() -----");
    for ch in s10.chars() {
        println!("{}", ch);
    }
    // バイト単位で取得
    println!("----- bytes() -----");
    for b in s10.bytes() {
        println!("{}", b);
    }
}
