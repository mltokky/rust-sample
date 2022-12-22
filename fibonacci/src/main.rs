use std::io::{self, Write};

fn main() {
    let number: u32;
    loop {
        print!("input number (> 0): ");
        io::stdout().flush().expect("flush error");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("cannot read line from stdin");
        match input.trim().parse() {
            Ok(num) => {
                number = num;
                break;
            },
            Err(_) => {}
        }
    }

    let mut before: u128 = 0;
    let mut current: u128 = 1;
    for _ in 1..number + 1 {
        println!("{}", current);
        let tmp = current;
        current = before + current;
        before = tmp;
    }
}
