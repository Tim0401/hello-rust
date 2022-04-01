extern crate test_learn;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_learn::add_two(2));
}
