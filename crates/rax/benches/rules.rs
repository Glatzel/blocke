use std::str::FromStr;

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rax::str_parser::IStrFlowRule;
use rax::str_parser::filters::{ASCII_LETTERS_DIGITS, CharSetFilter, DIGITS};
use rax::str_parser::rules::{
    ByteCount, Char, CharCount, NInCharSet, OneOfCharSet, Until, UntilNInCharSet,
    UntilNotInCharSet, UntilOneInCharSet,
};

fn bench_byte_count(c: &mut Criterion) {
    let rule = ByteCount::<2>;
    c.bench_function("byte_count", |b| b.iter(|| rule.apply(black_box("hello"))));
}
fn bench_char_count(c: &mut Criterion) {
    let rule = CharCount::<2>;
    c.bench_function("char_count", |b| {
        b.iter(|| rule.apply(black_box("110324,foo,bar")))
    });
}
fn bench_char(c: &mut Criterion) {
    let rule = Char::<'a'>;
    c.bench_function("char", |b| b.iter(|| rule.apply(black_box("a123"))));
}
fn bench_n_in_char_set(c: &mut Criterion) {
    let rule = NInCharSet::<3, 62>(&ASCII_LETTERS_DIGITS);
    c.bench_function("n_in_char_set", |b| {
        b.iter(|| rule.apply(black_box("abc123")))
    });
}
fn bench_one_in_char_set(c: &mut Criterion) {
    let rule = OneOfCharSet(&ASCII_LETTERS_DIGITS);
    c.bench_function("one_in_char_set", |b| {
        b.iter(|| rule.apply(black_box("a123")))
    });
}
fn bench_until_n_in_char_set(c: &mut Criterion) {
    let rule = UntilNInCharSet::<2, 10> {
        filter: &DIGITS,
        include: false,
    };
    c.bench_function("until_n_in_char_set", |b| {
        b.iter(|| rule.apply(black_box("a1b2c3")))
    });
}
fn bench_until_not_in_char_set(c: &mut Criterion) {
    let rule = UntilNotInCharSet {
        filter: &DIGITS,
        include: false,
    };
    c.bench_function("until_not_in_char_set", |b| {
        b.iter(|| rule.apply(black_box("123abc")))
    });
}
fn bench_until_one_in_char_set(c: &mut Criterion) {
    let filter = CharSetFilter::<2>::from_str(",*").unwrap();
    let rule = UntilOneInCharSet {
        filter: &filter,
        include: true,
    };
    c.bench_function("until_not_in_char_set", |b| {
        b.iter(|| rule.apply(black_box("0.7,1*38")))
    });
}
fn bench_until(c: &mut Criterion) {
    let rule = Until {
        delimiter: ";",
        include: false,
    };
    c.bench_function("until", |b| b.iter(|| rule.apply(black_box("123abc"))));
}
criterion_group!(
    benches,
    bench_byte_count,
    bench_char_count,
    bench_char,
    bench_n_in_char_set,
    bench_one_in_char_set,
    bench_until_n_in_char_set,
    bench_until_not_in_char_set,
    bench_until_one_in_char_set,
    bench_until
);
criterion_main!(benches);
