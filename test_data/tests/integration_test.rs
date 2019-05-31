use test_data;

mod common;
//断言是否匹配特定的错误描述
#[test]
#[should_panic(expected = "Guess value must be less than or equal to 100")]
fn greater_than_100() {
    common::setup();
    test_data::Guess::new(200);
}

