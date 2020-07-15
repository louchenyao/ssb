extern crate tblreader;

use tblreader::TBLReader;

#[derive(TBLReader)]
struct E {
    suppkey: Vec<i32>,
    name: Vec<i32>,
}

fn main() {}