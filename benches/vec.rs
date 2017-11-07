
#[macro_use]
extern crate bencher;

use bencher::Bencher;
use std::iter::repeat;

fn filter(ratio: u8, idx: usize) -> bool {
  idx % ratio as usize == 0
}

fn extend(ratio: u8, chars: &[char]) -> Vec<char> {
    let mut res = Vec::with_capacity(chars.len());
    res.extend(
        chars.iter().enumerate().filter_map(|(i, c)| {
            if filter(ratio, i) {
                Some(c)
            } else {
                None
            }
        })
    );
    res
}

fn collect(ratio: u8, chars: &[char]) -> Vec<char> {
    chars.iter().enumerate().filter_map(|(i, c)| {
        if filter(ratio, i) {
            Some(*c)
        } else {
            None
        }
    }).collect()
}

macro_rules! bench_gen {
    ($bench_name: ident, $inner: ident, $len: expr, $ratio: expr) => {
        fn $bench_name(b: &mut Bencher) {
            let chars: Vec<_> = repeat('a').take($len).collect();
            b.iter(|| $inner($ratio, &chars));
        }
    };
}

bench_gen!(collect_08_2, collect, 8, 2);
bench_gen!(collect_08_4, collect, 8, 4);
bench_gen!(collect_16_2, collect, 16, 2);
bench_gen!(collect_16_4, collect, 16, 4);
bench_gen!(collect_32_2, collect, 32, 2);
bench_gen!(collect_32_4, collect, 32, 4);
bench_gen!(extend_08_2, extend, 8, 2);
bench_gen!(extend_08_4, extend, 8, 4);
bench_gen!(extend_16_2, extend, 16, 2);
bench_gen!(extend_16_4, extend, 16, 4);
bench_gen!(extend_32_2, extend, 32, 2);
bench_gen!(extend_32_4, extend, 32, 4);

benchmark_group!(
    benches,
    collect_08_2,
    collect_08_4,
    collect_16_2,
    collect_16_4,
    collect_32_2,
    collect_32_4,
    extend_08_2,
    extend_08_4,
    extend_16_2,
    extend_16_4,
    extend_32_2,
    extend_32_4
);
benchmark_main!(benches);
