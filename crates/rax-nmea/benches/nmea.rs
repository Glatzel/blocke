use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rax::str_parser::StrParserContext;
use rax_nmea::data::{INmeaData, Talker};

fn bench_nmea<'a, F, D>(c: &mut Criterion, name: &str, sentence: &'static str, ctor: F)
where
    F: Fn(&mut StrParserContext, Talker) -> miette::Result<D> + 'static,
    D: INmeaData,
{
    let mut ctx = StrParserContext::new();
    ctx.init(sentence.to_string());
    c.bench_function(name, move |b| {
        b.iter(|| {
            ctx.reset();
            ctor(black_box(&mut ctx), black_box(Talker::GN)).unwrap();
        })
    });
}

pub fn benches(c: &mut Criterion) {
    bench_nmea(
        c,
        "dhv",
        "$GNDHV,021150.000,0.03,0.006,-0.042,-0.026,0.06*65",
        |ctx, t| rax_nmea::data::Dhv::new(ctx, t),
    );
    bench_nmea(
        c,
        "dtm",
        "$GPDTM,999,,0.08,N,0.07,E,-47.7,W84*1B",
        |ctx, t| rax_nmea::data::Dtm::new(ctx, t),
    );
    bench_nmea(c, "gbq", "$EIGBQ,RMC*28", |ctx, t| {
        rax_nmea::data::Gbq::new(ctx, t)
    });
    bench_nmea(
        c,
        "gbs",
        "$GPGBS,123519,0.9,0.8,1.2,1.0,1.0,1.0*41",
        |ctx, t| rax_nmea::data::Gbs::new(ctx, t),
    );
    bench_nmea(
        c,
        "gga",
        "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47",
        |ctx, t| rax_nmea::data::Gga::new(ctx, t),
    );
    bench_nmea(
        c,
        "gll",
        "$GPGLL,4916.45,N,12311.12,W,225444,A,*1D",
        |ctx, t| rax_nmea::data::Gll::new(ctx, t),
    );
    bench_nmea(c, "glq", "$GLGLQ,RMC*28", |ctx, t| {
        rax_nmea::data::Glq::new(ctx, t)
    });
    bench_nmea(c, "gns", "$GLGLQ,RMC*28", |ctx, t| {
        rax_nmea::data::Gns::new(ctx, t)
    });
    bench_nmea(c, "gpq", "$GLGLQ,RMC*28", |ctx, t| {
        rax_nmea::data::Gpq::new(ctx, t)
    });
    bench_nmea(c, "grs", "$GLGLQ,RMC*28", |ctx, t| {
        rax_nmea::data::Grs::new(ctx, t)
    });
    bench_nmea(
        c,
        "gsa",
        "$GPGSA,A,3,04,05,,09,12,,,,,1.8,1.0,1.2*30",
        |ctx, t| rax_nmea::data::Gsa::new(ctx, t),
    );
    bench_nmea(c, "grt", "$GLGLQ,RMC*28", |ctx, t| {
        rax_nmea::data::Gst::new(ctx, t)
    });
    bench_nmea(
        c,
        "gsv",
        "$GPGSV,2,1,08,01,40,083,41,02,17,308,43,03,23,120,42,04,10,180,39*75",
        |ctx, t| rax_nmea::data::Gsv::new(ctx, t),
    );
    bench_nmea(
        c,
        "rmc",
        "$GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A",
        |ctx, t| rax_nmea::data::Rmc::new(ctx, t),
    );

    bench_nmea(
        c,
        "ths",
        "$GPVTG,054.7,T,034.4,M,005.5,N,010.2,K*48",
        |ctx, t| rax_nmea::data::Ths::new(ctx, t),
    );
    bench_nmea(
        c,
        "txt",
        "$GPVTG,054.7,T,034.4,M,005.5,N,010.2,K*48",
        |ctx, t| rax_nmea::data::Txt::new(ctx, t),
    );
    bench_nmea(
        c,
        "vlw",
        "$GPVTG,054.7,T,034.4,M,005.5,N,010.2,K*48",
        |ctx, t| rax_nmea::data::Vlw::new(ctx, t),
    );
    bench_nmea(
        c,
        "vtg",
        "$GPVTG,054.7,T,034.4,M,005.5,N,010.2,K*48",
        |ctx, t| rax_nmea::data::Vtg::new(ctx, t),
    );
    bench_nmea(
        c,
        "zda",
        "$GPZDA,201530.00,04,07,2002,00,00*60",
        |ctx, t| rax_nmea::data::Zda::new(ctx, t),
    );
}

criterion_group!(benches_group, benches);
criterion_main!(benches_group);
