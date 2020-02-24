use tests;
mod common; // common code for multiple integration files.

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, tests::add_two(2));
}