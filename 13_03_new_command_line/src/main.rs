extern crate new_command_line;

use std::process;
use std::env;

use new_command_line::Config;
use new_command_line::run;

fn main() {
    // 失敗時の処理をクロージャーに任せる
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // エラーを標準エラーとして表示する
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Err値を返していたら中の処理を実行
    // ここではOkの時に返す値はないので、 `unwrap_or_else` よりもこちらを使うほうが良い
    if let Err(e) = run(config) {
        // エラーを標準エラーとして出力する
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}