use criterion::{black_box, criterion_group, criterion_main, Criterion};
use is_ascii_speedups::IsAscii2;
use rand::{Rng, thread_rng};
use rand::distributions::Standard;

macro_rules! ascii_bench {
    ($c: ident, $random_u8s: ident, $random_chars: ident, $ascii_u8s: ident, $ascii_chars: ident, $sorted_u8s: ident, $sorted_chars: ident, $name: ident, $one: ident, $two: ident) => {
        $c.bench_function(concat!(stringify!($name), "_u8_1_random"), |b| {
            b.iter(|| black_box($random_u8s.next().unwrap()).$one())
        });

        $c.bench_function(concat!(stringify!($name), "_u8_2_random"), |b| {
            b.iter(|| black_box($random_u8s.next().unwrap()).$two())
        });

        $c.bench_function(concat!(stringify!($name), "_u8_1_random_ascii"), |b| {
            b.iter(|| black_box($ascii_u8s.next().unwrap()).$one())
        });

        $c.bench_function(concat!(stringify!($name), "_u8_2_random_ascii"), |b| {
            b.iter(|| black_box($ascii_u8s.next().unwrap()).$two())
        });

        $c.bench_function(concat!(stringify!($name), "_u8_1_sorted_sequence"), |b| {
            b.iter(|| black_box($sorted_u8s.next().unwrap()).$one())
        });

        $c.bench_function(concat!(stringify!($name), "_u8_2_sorted_sequence"), |b| {
            b.iter(|| black_box($sorted_u8s.next().unwrap()).$two())
        });

        $c.bench_function(concat!(stringify!($name), "_char_1_random"), |b| {
            b.iter(|| black_box($random_chars.next().unwrap()).$one())
        });

        $c.bench_function(concat!(stringify!($name), "_char_2_random"), |b| {
            b.iter(|| black_box($random_chars.next().unwrap()).$two())
        });

        $c.bench_function(concat!(stringify!($name), "_char_1_random_ascii"), |b| {
            b.iter(|| black_box($ascii_chars.next().unwrap()).$one())
        });

        $c.bench_function(concat!(stringify!($name), "_char_2_random_ascii"), |b| {
            b.iter(|| black_box($ascii_chars.next().unwrap()).$two())
        });

        $c.bench_function(concat!(stringify!($name), "_char_1_sorted_sequence"), |b| {
            b.iter(|| black_box($sorted_chars.next().unwrap()).$one())
        });

        $c.bench_function(concat!(stringify!($name), "_char_2_sorted_sequence"), |b| {
            b.iter(|| black_box($sorted_chars.next().unwrap()).$two())
        });
    };
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut random_u8s = thread_rng().sample_iter::<u8, Standard>(Standard);
    let mut ascii_u8s = thread_rng().sample_iter::<u8, Standard>(Standard).map(|ch| ch & 0b0111_1111);
    let mut sorted_u8s = (0..=u8::MAX).cycle();

    let mut random_chars = thread_rng().sample_iter::<u32, Standard>(Standard)
        .map(|mut ch| {
            ch %= 0x0010_f800;
            ch += if ch < 0x0000_d800 { 0 } else { 0x0000_0800 };
            unsafe { char::from_u32_unchecked(ch) }
        });
    let mut ascii_chars = thread_rng().sample_iter::<u8, Standard>(Standard).map(|ch| char::from(ch & 0b0111_1111));
    let mut sorted_chars = ('\0'..='\u{d7ff}').chain('\u{e000}'..='\u{10ffff}').cycle();

    ascii_bench!(
        c,
        random_u8s,
        random_chars,
        ascii_u8s,
        ascii_chars,
        sorted_u8s,
        sorted_chars,
        ascii,
        is_ascii,
        is_ascii_2
    );

    ascii_bench!(
        c,
        random_u8s,
        random_chars,
        ascii_u8s,
        ascii_chars,
        sorted_u8s,
        sorted_chars,
        alphabetic,
        is_ascii_alphabetic,
        is_ascii_alphabetic_2
    );

    ascii_bench!(
        c,
        random_u8s,
        random_chars,
        ascii_u8s,
        ascii_chars,
        sorted_u8s,
        sorted_chars,
        alphanumeric,
        is_ascii_alphanumeric,
        is_ascii_alphanumeric_2
    );

    ascii_bench!(
        c,
        random_u8s,
        random_chars,
        ascii_u8s,
        ascii_chars,
        sorted_u8s,
        sorted_chars,
        control,
        is_ascii_control,
        is_ascii_control_2
    );

    ascii_bench!(
        c,
        random_u8s,
        random_chars,
        ascii_u8s,
        ascii_chars,
        sorted_u8s,
        sorted_chars,
        digit,
        is_ascii_digit,
        is_ascii_digit_2
    );

    ascii_bench!(
        c,
        random_u8s,
        random_chars,
        ascii_u8s,
        ascii_chars,
        sorted_u8s,
        sorted_chars,
        graphic,
        is_ascii_graphic,
        is_ascii_graphic_2
    );

    ascii_bench!(
        c,
        random_u8s,
        random_chars,
        ascii_u8s,
        ascii_chars,
        sorted_u8s,
        sorted_chars,
        hexdigit,
        is_ascii_hexdigit,
        is_ascii_hexdigit_2
    );

    ascii_bench!(
        c,
        random_u8s,
        random_chars,
        ascii_u8s,
        ascii_chars,
        sorted_u8s,
        sorted_chars,
        lowercase,
        is_ascii_lowercase,
        is_ascii_lowercase_2
    );

    ascii_bench!(
        c,
        random_u8s,
        random_chars,
        ascii_u8s,
        ascii_chars,
        sorted_u8s,
        sorted_chars,
        punctuation,
        is_ascii_punctuation,
        is_ascii_punctuation_2
    );

    ascii_bench!(
        c,
        random_u8s,
        random_chars,
        ascii_u8s,
        ascii_chars,
        sorted_u8s,
        sorted_chars,
        uppercase,
        is_ascii_uppercase,
        is_ascii_uppercase_2
    );

    ascii_bench!(
        c,
        random_u8s,
        random_chars,
        ascii_u8s,
        ascii_chars,
        sorted_u8s,
        sorted_chars,
        whitespace,
        is_ascii_whitespace,
        is_ascii_whitespace_2
    );
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark
}
criterion_main!(benches);
