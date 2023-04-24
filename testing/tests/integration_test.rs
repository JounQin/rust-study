use testing;

mod common;

#[test]
fn it_adds_two() {
    common::steup();
    assert_eq!(4, testing::add_two(2));
}
