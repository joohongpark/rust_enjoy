extern crate adder;

mod comm;

#[test]
fn integral_test() {
    let num = comm::get_num();
    assert_eq!(num, 1234);
}