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
    c.bench_function("dtm", |b| {
        b.iter(|| {
            ctx.reset();
            rax_nmea::data::Dtm::new(black_box(&mut ctx), black_box(Talker::GN)).unwrap();
        })
    });
}
fn bench_gbq(c: &mut Criterion) {
    let mut ctx = StrParserContext::new();
    ctx.init("$EIGBQ,RMC*28".to_string());
    c.bench_function("gbq", |b| {
        b.iter(|| {
            ctx.reset();
            rax_nmea::data::Gbq::new(black_box(&mut ctx), black_box(Talker::GN)).unwrap();
        })
    });
}
fn bench_gbs(c: &mut Criterion) {
    let mut ctx = StrParserContext::new();
    ctx.init("$GPGBS,123519,0.9,0.8,1.2,1.0,1.0,1.0*41".to_string());
    c.bench_function("gbs", |b| {
        b.iter(|| {
            ctx.reset();
            rax_nmea::data::Gbs::new(black_box(&mut ctx), black_box(Talker::GN)).unwrap();
        })
    });
}
fn bench_gga(c: &mut Criterion) {
    let mut ctx = StrParserContext::new();
    ctx.init("$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47".to_string());
    c.bench_function("gga", |b| {
        b.iter(|| {
            ctx.reset();
            rax_nmea::data::Gga::new(black_box(&mut ctx), black_box(Talker::GN)).unwrap();
        })
    });
}
fn bench_gll(c: &mut Criterion) {
    let mut ctx = StrParserContext::new();
    ctx.init("$GPGLL,4916.45,N,12311.12,W,225444,A,*1D".to_string());
    c.bench_function("gll", |b| {
        b.iter(|| {
            ctx.reset();
            rax_nmea::data::Gll::new(black_box(&mut ctx), black_box(Talker::GN)).unwrap();
        })
    });
}
fn bench_glq(c: &mut Criterion) {
    let mut ctx = StrParserContext::new();
    ctx.init("$GLGLQ,RMC*28".to_string());
    c.bench_function("glq", |b| {
        b.iter(|| {
            ctx.reset();
            rax_nmea::data::Glq::new(black_box(&mut ctx), black_box(Talker::GN)).unwrap();
        })
    });
}
fn bench_gns(c: &mut Criterion) {
    let mut ctx = StrParserContext::new();
    ctx.init("$GLGLQ,RMC*28".to_string());
    c.bench_function("gns", |b| {
        b.iter(|| {
            ctx.reset();
            rax_nmea::data::Gns::new(black_box(&mut ctx), black_box(Talker::GN)).unwrap();
        })
    });
}
fn bench_gsa(c: &mut Criterion) {
    let mut ctx = StrParserContext::new();
    ctx.init("$GPGSA,A,3,04,05,,09,12,,,,,1.8,1.0,1.2*30".to_string());
    c.bench_function("gsa", |b| {
        b.iter(|| {
            ctx.reset();
            rax_nmea::data::Gsa::new(black_box(&mut ctx), black_box(Talker::GN)).unwrap();
        })
    });
}
fn bench_gsv(c: &mut Criterion) {
    let mut ctx = StrParserContext::new();
    ctx.init("$GPGSV,2,1,08,01,40,083,41,02,17,308,43,03,23,120,42,04,10,180,39*75".to_string());
    c.bench_function("gsv", |b| {
        b.iter(|| {
            ctx.reset();
            rax_nmea::data::Gsv::new(black_box(&mut ctx), black_box(Talker::GN)).unwrap();
        })
    });
}
fn bench_rmc(c: &mut Criterion) {
    let mut ctx = StrParserContext::new();
    ctx.init("$GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A".to_string());
    c.bench_function("rmc", |b| {
        b.iter(|| {
            ctx.reset();
            rax_nmea::data::Rmc::new(black_box(&mut ctx), black_box(Talker::GN)).unwrap();
        })
    });
}
fn bench_vtg(c: &mut Criterion) {
    let mut ctx = StrParserContext::new();
    ctx.init("$GPVTG,054.7,T,034.4,M,005.5,N,010.2,K*48".to_string());
    c.bench_function("vtg", |b| {
        b.iter(|| {
            ctx.reset();
            rax_nmea::data::Vtg::new(black_box(&mut ctx), black_box(Talker::GN)).unwrap();
        })
    });
}
fn bench_zda(c: &mut Criterion) {
    let mut ctx = StrParserContext::new();
    ctx.init("$GPZDA,201530.00,04,07,2002,00,00*60".to_string());
    c.bench_function("zda", |b| {
        b.iter(|| {
            ctx.reset();
            rax_nmea::data::Zda::new(black_box(&mut ctx), black_box(Talker::GN)).unwrap();
        })
    });
}

criterion_group!(
    benches, bench_dhv, bench_dtm, bench_gbq, bench_gbs, bench_gga, bench_gll, bench_glq,
    bench_gsa, bench_gsv, bench_rmc, bench_vtg, bench_zda
);
criterion_main!(benches);
