fn main() {
    // Stackのような感じでdropの処理が積まれる
    let a = CustomSmartPointer { data: String::from("valueA") };
    let b = CustomSmartPointer { data: String::from("valueB") };

    println!("Created CustomSmartPointer.");
    // 手動でCustomSmartPointerを片付ける
    drop(a);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}