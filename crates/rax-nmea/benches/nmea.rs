use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rax::str_parser::StrParserContext;
use rax_nmea::data::{INmeaData, Talker};

fn bench_dhv(c: &mut Criterion) {
    let mut ctx = StrParserContext::new();
    ctx.init("$GNDHV,021150.000,0.03,0.006,-0.042,-0.026,0.06*65".to_string());
    c.bench_function("dhv", |b| {
        b.iter(|| {
            ctx.reset();
            rax_nmea::data::Dhv::new(black_box(&mut ctx), black_box(Talker::GN)).unwrap();
        })
    });
}
fn bench_dtm(c: &mut Criterion) {
    let mut ctx = StrParserContext::new();
    ctx.init("$GPDTM,999,,0.08,N,0.07,E,-47.7,W84*1B".to_string());
    c.bench_function("dhv", |b| {
        b.iter(|| {
            ctx.reset();
            rax_nmea::data::Dtm::new(black_box(&mut ctx), black_box(Talker::GN)).unwrap();
        })
    });
}
criterion_group!(benches, bench_dhv, bench_dtm);
criterion_main!(benches);
