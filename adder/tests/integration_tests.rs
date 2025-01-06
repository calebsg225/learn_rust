use adder::add_two;

mod common;

#[test]
fn it_adds_two_two() {
    common::setup();

    assert_eq!(add_two(3), 5);
}
