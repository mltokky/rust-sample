use std::ops::Deref;

fn main() {
    let x = 8;
    let y = MyBox::new(x);

    assert_eq!(8, x);
    assert_eq!(8, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// ポインタ外しのトレイトを実装したMyBox型
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}