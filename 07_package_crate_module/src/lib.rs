mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    // fn fix_incorrect_order() {
    //     cook_order();
    //     // superを使うと1つ親のmod内にある要素にアクセスできる
    //     super::serve_order();
    // }

    fn cook_order() {}
}

// 以下、どちらでも同じ
// use crate::front_of_house::hosting;
// use self::front_of_house::hosting;
// 以下でも可能だが、慣例的に関数までuseさせないことが一般的
// use crate::front_of_house::hosting::add_to_waitlist;
mod front_of_house;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // enumはenum自体にpubが入っていれば、以下の要素には全てアクセス可能
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // 絶対パスでアクセス → `crate` がルート
    crate::front_of_house::hosting::add_to_waitlist();
    // 相対パスでアクセス
    front_of_house::hosting::add_to_waitlist();
    // useでインポートされたスコープでアクセス
    hosting::add_to_waitlist();
}