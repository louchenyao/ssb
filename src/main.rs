use std::time::Instant;
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

#[derive(TBLReader)]
struct LO {
    orderkey: Vec<i32>,
    linenumber: Vec<i32>,
    custkey: Vec<i32>,
    partkey: Vec<i32>,
    suppkey: Vec<i32>,
    orderdate: Vec<i32>,
    ordpriority: Vec<String>,
    shippriority: Vec<i32>,
    quantity: Vec<i32>,
    extendedprice: Vec<i32>,
    ordtotalprice: Vec<i32>,
    discount: Vec<i32>,
    revenue: Vec<i32>,
    supplycost: Vec<i32>,
    tax: Vec<i32>,
    commitdate: Vec<i32>,
    shipmode: Vec<String>,
}

#[derive(TBLReader)]
struct D {
    datekey: Vec<i32>,
    date: Vec<String>,
    dayofweek: Vec<String>,
    month: Vec<String>,
    year: Vec<i32>,
    yearmonthnum: Vec<i32>,
    yearmonth: Vec<String>,
    daynuminweek: Vec<i32>,
    daynuminmonth: Vec<i32>,
    daynuminyear: Vec<i32>,
    monthnuminweek: Vec<i32>,
    weeknuminyear: Vec<i32>,
    sellingseason: Vec<String>,
    lastdayinmonth: Vec<i32>,
    holidayfl: Vec<i32>,
    weekdayfl: Vec<i32>,
    daynuminyer: Vec<i32>,
}

#[derive(TBLReader)]
struct P {
    partkey: Vec<i32>,
    name: Vec<String>,
    mfgr: Vec<String>,
    category: Vec<String>,
    brand1: Vec<String>,
    color: Vec<String>,
    type_: Vec<String>,
    size: Vec<i32>,
    container: Vec<String>,
}

#[derive(TBLReader)]
struct C {
    custkey: Vec<i32>,
    name: Vec<String>,
    address: Vec<String>,
    city: Vec<String>,
    nation: Vec<String>,
    region: Vec<String>,
    phone: Vec<String>,
    mktsegment: Vec<String>,
}

#[derive(Debug)]
struct Q1Res {
    revenue: i64,
}

// for SF = 1, revenue = 445921715901
fn q11(lo: &LO, d: &D) -> Q1Res {
    use std::collections::HashMap;
    
    let mut r = Q1Res{revenue: 0};

    // build
    let mut ht = HashMap::new();
    for (i, d_year) in d.year.iter().enumerate() {
        if d_year == &1993 {
            ht.insert(d.datekey[i], true);
        }
    }

    // probe
    for (i, lo_discount) in lo.discount.iter().enumerate() {
        if lo_discount >= &1 && lo_discount <= &3 && lo.quantity[i] < 25 {
            match ht.get(&lo.orderdate[i]) {
                Some(_) => {
                    r.revenue += (lo.extendedprice[i] as i64) * (lo.discount[i] as i64);
                },
                _ => {},
            }
        }
    }
    r
}

// for SF = 1, revenue = 97884685311
fn q12(lo: &LO, d: &D) -> Q1Res {
    use std::collections::HashMap;
    
    let mut r = Q1Res{revenue: 0};

    // build
    let mut ht = HashMap::new();
    for (i, d_yearmonthnum) in d.yearmonthnum.iter().enumerate() {
        if d_yearmonthnum == &199401 {
            ht.insert(d.datekey[i], true);
        }
    }

    // probe
    for (i, lo_discount) in lo.discount.iter().enumerate() {
        if lo_discount >= &4 && lo_discount <= &6 && lo.quantity[i] >= 26 && lo.quantity[i] <= 35 {
            match ht.get(&lo.orderdate[i]) {
                Some(_) => {
                    r.revenue += (lo.extendedprice[i] as i64) * (lo.discount[i] as i64);
                },
                _ => {},
            }
        }
    }
    r
}

// for SF = 1, revenue = 27885895351
fn q13(lo: &LO, d: &D) -> Q1Res {
    use std::collections::HashMap;
    
    let mut r = Q1Res{revenue: 0};

    // build
    let mut ht = HashMap::new();
    for (i, d_weeknuminyear) in d.weeknuminyear.iter().enumerate() {
        if d_weeknuminyear == &6 && d.year[i] == 1994 {
            ht.insert(d.datekey[i], true);
        }
    }

    // probe
    for (i, lo_discount) in lo.discount.iter().enumerate() {
        if lo_discount >= &5 && lo_discount <= &7 && lo.quantity[i] >= 26 && lo.quantity[i] <= 35 {
            match ht.get(&lo.orderdate[i]) {
                Some(_) => {
                    r.revenue += (lo.extendedprice[i] as i64) * (lo.discount[i] as i64);
                },
                _ => {},
            }
        }
    }
    r
}

#[derive(Debug)]
struct Q2Res {
    revenue: Vec<i64>,
    d_year: Vec<i32>,
    p_brand1: Vec<String>,
}

// for SF = 1, row_count = 280
fn q21(lo: &LO, d: &D, p: &P, s: &S) -> Q2Res {
    use std::collections::HashMap;
    let mut r = Q2Res{revenue: Vec::new(), d_year: Vec::new(), p_brand1: Vec::new()};

    // build date hash table
    let mut d_ht = HashMap::new();
    for (i, d_datekey) in d.datekey.iter().enumerate() {
        d_ht.insert(d_datekey, d.year[i]);
    }
    // build part hash table
    let mut p_ht = HashMap::new();
    for (i, p_category) in p.category.iter().enumerate() {
        if p_category == &"MFGR#12" {
            p_ht.insert(p.partkey[i], &p.brand1[i]);
        }
    }
    // build supplier hash table
    let mut s_ht = HashMap::new();
    for (i, s_region) in s.region.iter().enumerate() {
        if s_region == &"AMERICA" {
            s_ht.insert(s.suppkey[i], true);
        }
    }

    // probe and aggregate
    let mut res_ht = HashMap::<(i32, &str), i64>::new();
    for (i, lo_partkey) in lo.partkey.iter().enumerate() {
        if let Some(p_brand1) = p_ht.get(lo_partkey) {
            if let Some(_) = s_ht.get(&lo.suppkey[i]) {
                if let Some(d_year) = d_ht.get(&lo.orderdate[i]) {
                    *res_ht.entry((*d_year, *p_brand1)).or_insert(0) += lo.revenue[i] as i64;
                }
            }
        }
    }

    let mut v: Vec<_> = res_ht.into_iter().collect();
    v.sort();
    for ((d_year, p_brand1), revenue) in v {
        r.revenue.push(revenue);
        r.d_year.push(d_year);
        r.p_brand1.push(p_brand1.to_string());
    }

    r
}

// for SF = 1, row_count = 56
fn q22(lo: &LO, d: &D, p: &P, s: &S) -> Q2Res {
    use std::collections::HashMap;
    let mut r = Q2Res{revenue: Vec::new(), d_year: Vec::new(), p_brand1: Vec::new()};

    // build date hash table
    let mut d_ht = HashMap::new();
    for (i, d_datekey) in d.datekey.iter().enumerate() {
        d_ht.insert(d_datekey, d.year[i]);
    }
    // build part hash table
    let mut p_ht = HashMap::new();
    for (i, p_brand1) in p.brand1.iter().enumerate() {
        if p_brand1.as_str() >= "MFGR#2221" && p_brand1.as_str() <= "MFGR#2228" {
            p_ht.insert(p.partkey[i], &p.brand1[i]);
        }
    }
    // build supplier hash table
    let mut s_ht = HashMap::new();
    for (i, s_region) in s.region.iter().enumerate() {
        if s_region == &"ASIA" {
            s_ht.insert(s.suppkey[i], true);
        }
    }

    // probe and aggregate
    let mut res_ht = HashMap::<(i32, &str), i64>::new();
    for (i, lo_partkey) in lo.partkey.iter().enumerate() {
        if let Some(p_brand1) = p_ht.get(lo_partkey) {
            if let Some(_) = s_ht.get(&lo.suppkey[i]) {
                if let Some(d_year) = d_ht.get(&lo.orderdate[i]) {
                    *res_ht.entry((*d_year, *p_brand1)).or_insert(0) += lo.revenue[i] as i64;
                }
            }
        }
    }

    let mut v: Vec<_> = res_ht.into_iter().collect();
    v.sort();
    for ((d_year, p_brand1), revenue) in v {
        r.revenue.push(revenue);
        r.d_year.push(d_year);
        r.p_brand1.push(p_brand1.to_string());
    }

    r
}

// for SF = 1, row_count = 7
fn q23(lo: &LO, d: &D, p: &P, s: &S) -> Q2Res {
    use std::collections::HashMap;
    let mut r = Q2Res{revenue: Vec::new(), d_year: Vec::new(), p_brand1: Vec::new()};

    // build date hash table
    let mut d_ht = HashMap::new();
    for (i, d_datekey) in d.datekey.iter().enumerate() {
        d_ht.insert(d_datekey, d.year[i]);
    }
    // build part hash table
    let mut p_ht = HashMap::new();
    for (i, p_brand1) in p.brand1.iter().enumerate() {
        if p_brand1.as_str() == "MFGR#2221" {
            p_ht.insert(p.partkey[i], &p.brand1[i]);
        }
    }
    // build supplier hash table
    let mut s_ht = HashMap::new();
    for (i, s_region) in s.region.iter().enumerate() {
        if s_region == &"EUROPE" {
            s_ht.insert(s.suppkey[i], true);
        }
    }

    // probe and aggregate
    let mut res_ht = HashMap::<(i32, &str), i64>::new();
    for (i, lo_partkey) in lo.partkey.iter().enumerate() {
        if let Some(p_brand1) = p_ht.get(lo_partkey) {
            if let Some(_) = s_ht.get(&lo.suppkey[i]) {
                if let Some(d_year) = d_ht.get(&lo.orderdate[i]) {
                    *res_ht.entry((*d_year, *p_brand1)).or_insert(0) += lo.revenue[i] as i64;
                }
            }
        }
    }

    let mut v: Vec<_> = res_ht.into_iter().collect();
    v.sort();
    for ((d_year, p_brand1), revenue) in v {
        r.revenue.push(revenue);
        r.d_year.push(d_year);
        r.p_brand1.push(p_brand1.to_string());
    }

    r
}

#[derive(Debug)]
struct Q31Res {
    c_nation: Vec<String>,
    s_nation: Vec<String>,
    d_year: Vec<i32>,
    revenue: Vec<i64>,
}

fn q31(c: &C, lo: &LO, s: &S, d: &D) -> Q31Res {
    use std::collections::HashMap;

    // build date hash table
    let mut d_ht = HashMap::new();
    for (i, d_year) in d.year.iter().enumerate() {
        if d_year >= &1992 && d_year <= &1997{
            d_ht.insert(d.datekey[i], d_year);
        }
    }
    // build supplier hash table
    let mut s_ht = HashMap::new();
    for (i, s_region) in s.region.iter().enumerate() {
        if s_region == &"ASIA" {
            s_ht.insert(s.suppkey[i], &s.nation[i]);
        }
    }
    // build customer hash table
    let mut c_ht = HashMap::new();
    for (i, c_region) in c.region.iter().enumerate() {
        if c_region == &"ASIA" {
            c_ht.insert(c.custkey[i], &c.nation[i]);
        }
    }

    // probe and aggregate
    let mut res_ht = HashMap::<(&str, &str, i32), i64>::new();
    for (i, lo_orderdate) in lo.orderdate.iter().enumerate() {
        if let Some(d_year) = d_ht.get(lo_orderdate) {
            if let Some(s_nation) = s_ht.get(&lo.suppkey[i]) {
                if let Some(c_nation) = c_ht.get(&lo.custkey[i]) {
                    *res_ht.entry((c_nation, s_nation, **d_year)).or_insert(0) += lo.revenue[i] as i64;
                }
            }
        }
    }

    let mut v: Vec<_> = res_ht.into_iter().collect();
    let mut r= Q31Res {c_nation: Vec::new(), s_nation: Vec::new(), d_year: Vec::new(), revenue: Vec::new()};
    //order by d_year asc, revenue desc; 
    v.sort_by_key(|x| ((x.0).2, -x.1));
    for ((c_nation, s_nation, d_year), revenue) in v {
        r.c_nation.push(c_nation.to_string());
        r.s_nation.push(s_nation.to_string());
        r.d_year.push(d_year);
        r.revenue.push(revenue);
    }
    r
}

fn main() {
    println!("Loading...");
    let start = Instant::now();
    let d = D::load("./ssb-dbgen/date.tbl");
    let lo = LO::load("./ssb-dbgen/lineorder.tbl");
    let s = S::load("./ssb-dbgen/supplier.tbl");
    let p = P::load("./ssb-dbgen/part.tbl");
    let c = C::load("./ssb-dbgen/customer.tbl");
    println!("Takes {} seconds to load.", start.elapsed().as_millis() as f32 / 1000.0);

    let start = Instant::now();
    println!("q11: {:?}", q11(&lo, &d));
    println!("q11 takes {} ms.", start.elapsed().as_millis());

    let start = Instant::now();
    println!("q12: {:?}", q12(&lo, &d));
    println!("q12 takes {} ms.", start.elapsed().as_millis());

    let start = Instant::now();
    println!("q13: {:?}", q13(&lo, &d));
    println!("q13 takes {} ms.", start.elapsed().as_millis());

    let start = Instant::now();
    let q21_r = q21(&lo, &d, &p, &s);
    println!("q21 takes {} ms.", start.elapsed().as_millis());
    println!("q21 row_count: {}", q21_r.d_year.len());
    //println!("Q21 res: {:?}", q21_r);

    let start = Instant::now();
    let q22_r = q22(&lo, &d, &p, &s);
    println!("q22 takes {} ms.", start.elapsed().as_millis());
    println!("Q22 row_count: {}", q22_r.d_year.len());
    //println!("Q22 res: {:?}", q22_r);

    let start = Instant::now();
    let q23_r = q23(&lo, &d, &p, &s);
    println!("q23 takes {} ms.", start.elapsed().as_millis());
    println!("q23 row_count: {}", q23_r.d_year.len());
    //println!("Q22 res: {:?}", q22_r);

    let start = Instant::now();
    let q31_r = q31(&c, &lo, &s, &d);
    println!("q31 takes {} ms.", start.elapsed().as_millis());
    println!("q31 row_count: {}", q31_r.d_year.len());
    println!("Q31 res: {:?}", q31_r);
}