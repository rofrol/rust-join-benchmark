#![feature(test)]

extern crate itertools;
extern crate test;

use itertools::Itertools;
use test::Bencher;

fn strings_vec() -> Vec<String> {
    vec![String::from("again"); 512]
}

#[bench]
fn consume_collect_join(b: &mut Bencher) {
    b.iter(|| {
        strings_vec()
            .into_iter()
            .collect::<Vec<String>>()
            .join(" and ")
    });
}

#[bench]
fn consume_itertools_join(b: &mut Bencher) {
    b.iter(|| strings_vec().into_iter().join(" and "));
}

#[bench]
fn borrow_collect_join(b: &mut Bencher) {
    let strings = strings_vec();
    b.iter(|| {
        strings
            .iter()
            .map(|s| &**s)
            .collect::<Vec<&str>>()
            .join(" and ")
    });
}

#[bench]
fn borrow_itertools_join(b: &mut Bencher) {
    let strings = strings_vec();
    b.iter(|| strings.iter().join(" and "));
}
