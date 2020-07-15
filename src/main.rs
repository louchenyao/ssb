use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


struct S {
    suppkey: Vec<i32>,
    name: Vec<String>,
    address: Vec<String>,
    city: Vec<String>,
    nation: Vec<String>,
    region: Vec<String>,
    phone: Vec<String>,
}

impl S {
    fn new() -> Self {
        Self {
            suppkey: Vec::new(),
            name: Vec::new(),
            address: Vec::new(),
            city: Vec::new(),
            nation: Vec::new(),
            region: Vec::new(),
            phone: Vec::new(),
        }
    }

    fn load<P>(&mut self, p: P) where P: AsRef<Path> {
        let reader = {
            let f =  File::open(p).unwrap();
            io::BufReader::new(f).lines()
        };
    
        for line in reader {
            if let Ok(l) = line {
                let v = l.split("|").collect::<Vec<&str>>();
                self.suppkey.push(v[0].parse::<i32>().unwrap());
                self.name.push(v[1].to_string());
                self.address.push(v[2].to_string());
                self.city.push(v[3].to_string());
                self.nation.push(v[4].to_string());
                self.region.push(v[5].to_string());
                self.phone.push(v[6].to_string());
            }
        }
    }
}

fn main() {
    let mut s = S::new();
    s.load("./ssb-dbgen/supplier.tbl");

    for c in s.suppkey {
        println!("{}", c);
    }
}