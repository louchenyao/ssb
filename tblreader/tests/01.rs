extern crate tblreader;

use tblreader::TBLReader;

#[derive(TBLReader)]
struct E {
    suppkey: Vec<i32>,
    name: Vec<String>,
    val: Vec<u8>,
}

#[test]
fn t() {
    let e = E::load("tests/example.tbl");
    assert_eq!(&e.suppkey, &vec![1, 233]);
    assert_eq!(&e.name, &vec!["hi", "b"]);
    assert_eq!(&e.val, &vec![12, 2]);
}

fn main() {}