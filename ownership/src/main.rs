fn main() {
    // 文字列リテラル
    let s1 = "文字列";
    println!("s1: {}", s1);
    // String型
    let mut s2 = String::from("文字列");
    s2.push_str("だよ！"); // 末尾に文字列を追加する
    println!("s2: {}", s2);

    // 所有権のMove
    let s3 = s2; // この時点でs1は使えなくなる

    // 文字列のdeep copy
    let s4 = s3.clone();
    println!("s3: {}", s3);
    println!("s4: {}", s4);

    // スカラー値の場合はMoveではなくCopyされる
    let s5 = s1;
    println!("s1: {}, s5: {}", s1, s5);

    // s4はこのメソッド以降は使えない（Moveしたので）
    print(s4);

    let s6 = String::from("単なる文字列");
    let s6_size = calculate_length(&s6); // s6は参照しただけなので所有権は移らず、以降も利用可能
    println!("s6: {} ({} bytes)", s6, s6_size);

    let mut s7 = String::from("s7という文字列");
    add_prefix(&mut s7);
    println!("s7: {}", s7);

    // スライス
    let hello_world = String::from("Hello world!");
    let hello = first_word(&hello_world);
    let world = second_word(&hello_world);
    println!("first: {}, second: {}", hello, world);
}

fn print(s: String) {
    println!("print -- {}", s);
}

fn calculate_length(s: &String) -> usize {
    // 参照としてsを受け取る → 所有権は移動しない
    s.len()
}

fn add_prefix(s: &mut String) {
    s.push_str(" Prefix!");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // <- 0〜iまでの配列の一部（スライス）を返す
        }
    }

    &s
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    let mut start_index = 0;
    let mut end_index = bytes.len();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if start_index == 0 {
                start_index = i + 1;
            } else {
                end_index = i;
                break;
            }
        }
    }

    &s[start_index..end_index]
}
