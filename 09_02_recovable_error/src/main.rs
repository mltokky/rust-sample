use std::{fs::File, io::{ErrorKind, Read, self}};

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("ファイル作成に失敗: {:?}", e);
                }
            }
        },
        Err(error) => {
            panic!("ファイルオープンに失敗: {:?}", error);
        }
    };

    // ファイルがないとpanic!となる
    // let f = File::open("hello2.txt").unwrap();

    // ファイルがないと指定されたメッセージでpanic!になる
    // let f = File::open("hello3.txt").expect("file open error");

    // ?演算子 を使う場合、呼び出し元関数の戻り値が Result型 じゃないとエラーになる
    // let f = read_username_from_file()?;

    println!("file content: {}", read_username_from_file().unwrap());
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    // ここでエラーが起きたならErrがそのまま返る
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    // Stringの読み取り成功→String型、失敗→Errが返る
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 上記メソッドの簡易版
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;  // ?演算子 - 処理の実行に失敗した場合は Err が自動的に返る
    let mut s = String::new();
    f.read_to_string(&mut s)?;  // ?演算子 - 処理の実行に失敗した場合は Err が自動的に返る

    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;  // read_username_from_file2() の内容を1つにまとめることも可能

    Ok(s)
}