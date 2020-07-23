use tblreader::TBLReader;

#[derive(TBLReader)]
struct S {
    suppkey: Vec<i32>,
    name: Vec<String>,
    address: Vec<String>,
    city: Vec<String>,
    nation: Vec<String>,
    region: Vec<String>,
    phone: Vec<String>,
}


fn main() {
    let s = S::load("./ssb-dbgen/supplier.tbl");

    for (i, c) in s.suppkey.iter().enumerate() {
        println!("{}", c);
        if i > 10 {
            break;
        }
    }
}