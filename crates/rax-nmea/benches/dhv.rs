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

criterion_group!(benches, bench_dhv);
criterion_main!(benches);
