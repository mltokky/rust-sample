pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
    // String::from("Hello")
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 0 {
            panic!(
                "Guess value must be larger than 0, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be smaller than 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // panic!マクロで必ず失敗するテスト
    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Rust");
        // カスタムのエラーメッセージ付き
        assert!(
            result.contains("Rust"),
            "Greeting did not contains name, value was {}",
            result
        );
    }

    // panicが起こることを想定するテスト
    #[test]
    #[should_panic(expected = "Guess value must be smaller than 100, got 200.")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "Guess value must be larger than 0, got -5.")]
    fn smaller_than_0() {
        Guess::new(-5);
    }
}
