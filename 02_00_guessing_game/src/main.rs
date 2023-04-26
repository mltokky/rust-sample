/*
 * 数当てゲームのプログラミング
 * https://doc.rust-jp.rs/book-ja/ch02-00-guessing-game-tutorial.html
 *
 */

use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number!");

    // 1〜100までの範囲式 ... 1..101 → 101を含まない、 1..=100 → 100を含む
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess_count: u32 = 0;

    loop {
        guess_count += 1;
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("guess count: {}", guess_count);
                break;
            }
        }
    }
}
