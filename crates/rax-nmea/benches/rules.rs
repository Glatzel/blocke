use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rax::str_parser::IStrFlowRule;
use rax_nmea::{NMEA_COORD, NMEA_DATE, NMEA_DEGREE};
fn bench_coord(c: &mut Criterion) {
    c.bench_function("coord", |b| {
        b.iter(|| NMEA_COORD.apply(black_box("12319.123,E,rest")))
    });
}
fn bench_date(c: &mut Criterion) {
    c.bench_function("date", |b| {
        b.iter(|| NMEA_DATE.apply(black_box("110324,foo,bar")))
    });
}
fn bench_degree(c: &mut Criterion) {
    c.bench_function("degree", |b| {
        b.iter(|| NMEA_DEGREE.apply(black_box("123.45,N,other_data")))
    });
}
fn bench_utc(c: &mut Criterion) {
    c.bench_function("utc", |b| {
        b.iter(|| NMEA_COORD.apply(black_box("123456.789,foo,bar")))
    });
}
fn bench_validate(c: &mut Criterion) {
    c.bench_function("validate", |b| {
        b.iter(|| {
            NMEA_COORD.apply(black_box(
                "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47",
            ))
        })
    });
}
criterion_group!(
    benches,
    bench_coord,
    bench_date,
    bench_degree,
    bench_utc,
    bench_validate
);
criterion_main!(benches);
