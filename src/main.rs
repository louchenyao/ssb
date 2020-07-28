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


macro_rules! ht {
    (@pred_expaned $tbl:expr, $idx:expr, $l:ident == $r:expr, and $($rest:tt)*) => {{
        ht![@pred_expaned $tbl, $idx, $l == $r] && ht![@pred_expaned $tbl, $idx, $($rest)*]
    }};
    (@pred_expaned $tbl:expr, $idx:expr, $l:ident >= $r:expr, and $($rest:tt)*) => {{
        ht![@pred_expaned $tbl, $idx, $l >= $r] && ht![@pred_expaned $tbl, $idx, $($rest)*]
    }};
    (@pred_expaned $tbl:expr, $idx:expr, $l:ident <= $r:expr, and $($rest:tt)*) => {{
        ht![@pred_expaned $tbl, $idx, $l <= $r] && ht![@pred_expaned $tbl, $idx, $($rest)*]
    }};
    (@pred_expaned $tbl:expr, $idx:expr, $l:ident == $r:expr) => {{
        $tbl.$l[$idx] == $r
    }};
    (@pred_expaned $tbl:expr, $idx:expr, $l:ident >= $r:expr) => {{
        $tbl.$l[$idx] >= $r
    }};
    (@pred_expaned $tbl:expr, $idx:expr, $l:ident <= $r:expr) => {{
        $tbl.$l[$idx] <= $r
    }};
    (@pred_expaned $tbl:expr, $idx:expr,) => {{
        true
    }};
    (@v_expaned $tbl:expr, $idx:expr, true) => {
        true
    };
    (@v_expaned $tbl:expr, $idx:expr, $v:ident) => {
        $tbl.$v[$idx]
    };
    ($tbl:expr; $k:ident => &$v:ident; $($pred:tt)*) => {{
        let mut t = std::collections::HashMap::new();
        for i in 0..$tbl.$k.len() {
            if ht![@pred_expaned $tbl, i, $($pred)*] {
                t.insert($tbl.$k[i], &ht![@v_expaned $tbl, i, $v]);
            }
        }
        t
    }};
    ($tbl:expr; $k:ident => $v:ident; $($pred:tt)*) => {{
        let mut t = std::collections::HashMap::new();
        for i in 0..$tbl.$k.len() {
            if ht![@pred_expaned $tbl, i, $($pred)*] {
                t.insert($tbl.$k[i], ht![@v_expaned $tbl, i, $v]);
            }
        }
        t
    }};
}

// for SF = 1, revenue = 445921715901
fn q11(lo: &LO, d: &D) -> Q1Res {
    // build
    let ht = ht![d; datekey => true; year == 1993];

    // probe
    let mut r = Q1Res{revenue: 0};
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
    // build
    let ht = ht![d; datekey => true; yearmonthnum == 199401];

    // probe
    let mut r = Q1Res{revenue: 0};
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
    // build
    let ht = ht![d; datekey => true; weeknuminyear == 6, and year == 1994];

    // probe
    let mut r = Q1Res{revenue: 0};
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

    // build hash tables
    let d_ht = ht![d; datekey => year;];
    let p_ht = ht![p; partkey => &brand1; category == "MFGR#12"];
    let s_ht = ht![s; suppkey => true; region == "AMERICA"];

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

    // build
    let d_ht = ht![d; datekey => year;];
    // &brand1 >= "MFGR#2221" is faster than brand1 >= "MFGR#2221".to_string()
    // but the ht! macro doesn't support that
    let p_ht = ht![p; partkey => &brand1; brand1 >= "MFGR#2221".to_string(), and brand1 <= "MFGR#2228".to_string()];
    let s_ht = ht![s; suppkey => true; region == "ASIA"];

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

    // build
    let d_ht = ht![d; datekey => year;];
    let p_ht = ht![p; partkey => &brand1; brand1 == "MFGR#2221"];
    let s_ht = ht![s; suppkey => true; region == "EUROPE"];

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

    // build
    let d_ht = ht![d; datekey => year; year >= 1992, and year <= 1997];
    let s_ht = ht![s; suppkey => &nation; region == "ASIA"];
    let c_ht = ht![c; custkey => &nation; region == "ASIA"];

    // probe and aggregate
    let mut res_ht = HashMap::<(&str, &str, i32), i64>::new();
    for (i, lo_orderdate) in lo.orderdate.iter().enumerate() {
        if let Some(d_year) = d_ht.get(lo_orderdate) {
            if let Some(s_nation) = s_ht.get(&lo.suppkey[i]) {
                if let Some(c_nation) = c_ht.get(&lo.custkey[i]) {
                    *res_ht.entry((c_nation, s_nation, *d_year)).or_insert(0) += lo.revenue[i] as i64;
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
    let q11_r = q11(&lo, &d);
    assert_eq!(q11_r.revenue, 445921715901);
    println!("q11 takes {} ms.", start.elapsed().as_millis());

    let start = Instant::now();
    let q12_r = q12(&lo, &d);
    assert_eq!(q12_r.revenue, 97884685311);
    println!("q12 takes {} ms.", start.elapsed().as_millis());

    let start = Instant::now();
    let q13_r = q13(&lo, &d);
    assert_eq!(q13_r.revenue, 27885895351);
    println!("q13 takes {} ms.", start.elapsed().as_millis());

    let start = Instant::now();
    let q21_r = q21(&lo, &d, &p, &s);
    assert_eq!(q21_r.d_year.len(), 280);
    println!("q21 takes {} ms.", start.elapsed().as_millis());

    let start = Instant::now();
    let q22_r = q22(&lo, &d, &p, &s);
    assert_eq!(q22_r.d_year.len(), 56);
    println!("q22 takes {} ms.", start.elapsed().as_millis());

    let start = Instant::now();
    let q23_r = q23(&lo, &d, &p, &s);
    assert_eq!(q23_r.d_year.len(), 7);
    println!("q23 takes {} ms.", start.elapsed().as_millis());

    let start = Instant::now();
    let q31_r = q31(&c, &lo, &s, &d);
    println!("q31 takes {} ms.", start.elapsed().as_millis());
    println!("q31 row_count: {}", q31_r.d_year.len());
    //println!("Q31 res: {:?}", q31_r);
}