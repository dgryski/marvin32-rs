use bencher::benchmark_group;
use bencher::Bencher;
use bencher::benchmark_main;
use marvin32_rs::hash;

macro_rules! bench_hash {
    ($name:ident, $size:expr) => {
        fn $name(b: &mut Bencher) {
            let mut val: u32 = 0;

            let mut v = Vec::<u8>::new();
            v.resize($size, 0);

            b.iter(|| {
                val += hash(0, &v);
            })
        }
    };
}

bench_hash!(benchmark_0008, 8);
bench_hash!(benchmark_0016, 16);
bench_hash!(benchmark_0032, 32);
bench_hash!(benchmark_0040, 40);
bench_hash!(benchmark_0060, 60);
bench_hash!(benchmark_0064, 64);
bench_hash!(benchmark_0072, 72);
bench_hash!(benchmark_0080, 80);
bench_hash!(benchmark_0100, 100);
bench_hash!(benchmark_0150, 150);
bench_hash!(benchmark_0200, 200);
bench_hash!(benchmark_0250, 250);
bench_hash!(benchmark_0512, 512);
bench_hash!(benchmark_1024, 1024);
bench_hash!(benchmark_8192, 8192);


benchmark_group!(benches,
    benchmark_0008,
    benchmark_0016,
    benchmark_0032,
    benchmark_0040,
    benchmark_0060,
    benchmark_0064,
    benchmark_0072,
    benchmark_0080,
    benchmark_0100,
    benchmark_0150,
    benchmark_0200,
    benchmark_0250,
    benchmark_0512,
    benchmark_1024,
    benchmark_8192
);
benchmark_main!(benches);