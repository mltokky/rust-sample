extern crate auto_test;

#[test]
fn it_adds_two() {
    assert_eq!(4, auto_test::add(1, 3));
}