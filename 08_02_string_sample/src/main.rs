/*
 * 8.2. 文字列でUTF-8でエンコードされたテキストを保持する
 * https://doc.rust-jp.rs/book-ja/ch08-02-strings.html
 */

fn main() {
    let s = String::from("これは文字列");
    println!("String::from(): {}\n", s);

    // 文字列スライス → Stringへ変換
    let data = "initial string data";
    let s = data.to_string();
    println!("data.to_string(): {}\n", s);

    println!("----- UTF-8 encodeded characters -----");
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
    println!();

    // Stringに新たに文字列を追加
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!("push_str(): {}", s);
    // strのスライスを渡したので、所有権は奪っていない → 使用可能
    println!("s2 is {}", s2);
    println!();

    let s1 = String::from("Hello, ");
    let s2 = String::from("Rust world!");
    // ジェネリクスで定義された add() 関数が呼ばれている
    let s3 = s1 + &s2;
    println!("s1 + s2: {}", s3);
    println!();

    // 複雑な文字列の連結にはformat!マクロを使う　所有権も奪わない
    let s1 = String::from("stone");
    let s2 = String::from("sky");
    let s3 = String::from("sea");
    let s4 = format!("{}-{}-{}", s1, s2, s3);
    println!("use format! macro: {}", s4);
    println!();

    let s = "日本語で文字列を打ちます";
    // let s_slice = &s[0..4];  // これはエラー：マルチバイト文字の途中の位置（'本'のインデックスが3..6であり、その間までをスライスにしようとしたため）
    // println!("s slice: {}", s_slice);
    // 正確に1文字ずつアクセスするには chars() を使用する
    println!("Japanese characters: {}", s);
    print!("-> chars(): ");
    for ch in s.chars() {
        print!("{} ", ch);
    }
    println!();

    // バイト単位で取得
    print!("-> bytes(): ");
    for b in s.bytes() {
        print!("{} ", b);
    }
    println!();
}
