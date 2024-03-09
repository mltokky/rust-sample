use std::{collections::HashMap, iter::Map, thread, time::Duration};

fn main() {
    generate_workout(10, 7);
}

fn generate_workout(intensity: u32, random_number: u32) {
    // クロージャーを変数に保存する → Rubyと定義が似ている！
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    // クロージャーの引数が2つ以上ある場合は `,` でつなげる
    // let expensive_closure2 = |num1: u32, num2: u32| {
    //     println!("calculating very slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num1 + num2
    // };


    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure.value(intensity));
        }
    }

    // クロージャーの内容が1行で終わる場合は `{}` も省略可能
    // let add_two_numbers = |num1, num2| num1 + num2;
    // println!("1 + 4 = {}", add_two_numbers(1, 4));
}

// ジェネリクス `T` のトレイト境界：Fnトレイト（クロージャー）
struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            },
        }
    }
}