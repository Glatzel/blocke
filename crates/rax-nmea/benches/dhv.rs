use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rax::str_parser::StrParserContext;
use rax_nmea::data::{INmeaData, Talker};

fn bench_datum_compensate(c: &mut Criterion) {
    let mut ctx = StrParserContext::new();
    ctx.init("$GNDHV,021150.000,0.03,0.006,-0.042,-0.026,0.06*65".to_string());
    c.bench_function("datum_compensate", |b| {
        for _ in 0..100_000 {
            ctx.reset();
            b.iter(|| rax_nmea::data::Dhv::new(black_box(&mut ctx), black_box(Talker::GN)))
        }
    });
}

criterion_group!(benches, bench_datum_compensate);
criterion_main!(benches);
