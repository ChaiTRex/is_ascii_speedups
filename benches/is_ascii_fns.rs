use criterion::{black_box, criterion_group, criterion_main, Criterion};
use is_ascii_speedups::IsAscii2;
use rand::distributions::Standard;
use rand::{thread_rng, Rng};

macro_rules! ascii_benches {
    (
        $name: ident,
        $this_method_u8s: ident,
        $old_method: ident,
        $new_method: ident,
        $c: ident,
        $ascii_u8s: ident,
        $random_u8s: ident,
        $random_chars: ident,
        $u8_string: ident,
        $char_string: ident,
    ) => {
        $c.bench_function(concat!("old_", stringify!($name), "_random_u8s"), |b| {
            b.iter(|| black_box($random_u8s.next().unwrap()).$old_method())
        });

        $c.bench_function(concat!("new_", stringify!($name), "_random_u8s"), |b| {
            b.iter(|| black_box($random_u8s.next().unwrap()).$new_method())
        });

        $c.bench_function(
            concat!("old_", stringify!($name), "_random_ascii_u8s"),
            |b| b.iter(|| black_box($ascii_u8s.next().unwrap()).$old_method()),
        );

        $c.bench_function(
            concat!("new_", stringify!($name), "_random_ascii_u8s"),
            |b| b.iter(|| black_box($ascii_u8s.next().unwrap()).$new_method()),
        );

        $c.bench_function(
            concat!("old_", stringify!($name), "_count_starting_u8_matches"),
            |b| {
                b.iter(|| {
                    let length = ($random_u8s.next().unwrap() % 65) as usize;
                    $u8_string.clear();
                    $u8_string.extend($this_method_u8s.by_ref().take(length));
                    $u8_string.extend($random_u8s.by_ref().take(64 - length));
                    black_box($u8_string.iter().take_while(|ch| ch.$old_method()).count())
                })
            },
        );

        $c.bench_function(
            concat!("new_", stringify!($name), "_count_starting_u8_matches"),
            |b| {
                b.iter(|| {
                    let length = ($random_u8s.next().unwrap() % 65) as usize;
                    $u8_string.clear();
                    $u8_string.extend($this_method_u8s.by_ref().take(length));
                    $u8_string.extend($random_u8s.by_ref().take(64 - length));
                    black_box($u8_string.iter().take_while(|ch| ch.$new_method()).count())
                })
            },
        );

        $c.bench_function(
            concat!("old_", stringify!($name), "_count_all_u8_matches"),
            |b| {
                b.iter(|| {
                    let length = ($random_u8s.next().unwrap() % 65) as usize;
                    $u8_string.clear();
                    $u8_string.extend($this_method_u8s.by_ref().take(length));
                    $u8_string.extend($random_u8s.by_ref().take(64 - length));
                    black_box($u8_string.iter().filter(|ch| ch.$old_method()).count())
                })
            },
        );

        $c.bench_function(
            concat!("new_", stringify!($name), "_count_all_u8_matches"),
            |b| {
                b.iter(|| {
                    let length = ($random_u8s.next().unwrap() % 65) as usize;
                    $u8_string.clear();
                    $u8_string.extend($this_method_u8s.by_ref().take(length));
                    $u8_string.extend($random_u8s.by_ref().take(64 - length));
                    black_box($u8_string.iter().filter(|ch| ch.$new_method()).count())
                })
            },
        );

        $c.bench_function(concat!("old_", stringify!($name), "_random_chars"), |b| {
            b.iter(|| black_box($random_chars.next().unwrap()).$old_method())
        });

        $c.bench_function(concat!("new_", stringify!($name), "_random_chars"), |b| {
            b.iter(|| black_box($random_chars.next().unwrap()).$new_method())
        });

        $c.bench_function(
            concat!("old_", stringify!($name), "_random_ascii_chars"),
            |b| {
                b.iter(|| {
                    black_box($ascii_u8s.by_ref().map(|ch| char::from(ch)).next().unwrap())
                        .$old_method()
                })
            },
        );

        $c.bench_function(
            concat!("new_", stringify!($name), "_random_ascii_chars"),
            |b| {
                b.iter(|| {
                    black_box($ascii_u8s.by_ref().map(|ch| char::from(ch)).next().unwrap())
                        .$new_method()
                })
            },
        );

        $c.bench_function(
            concat!("old_", stringify!($name), "_count_starting_char_matches"),
            |b| {
                b.iter(|| {
                    let length = ($random_u8s.next().unwrap() % 65) as usize;
                    $char_string.clear();
                    $char_string.extend(
                        $this_method_u8s
                            .by_ref()
                            .map(|ch| char::from(ch))
                            .take(length),
                    );
                    $char_string.extend($random_chars.by_ref().take(64 - length));
                    black_box(
                        $char_string
                            .chars()
                            .take_while(|ch| ch.$old_method())
                            .count(),
                    )
                })
            },
        );

        $c.bench_function(
            concat!("new_", stringify!($name), "_count_starting_char_matches"),
            |b| {
                b.iter(|| {
                    let length = ($random_u8s.next().unwrap() % 65) as usize;
                    $char_string.clear();
                    $char_string.extend(
                        $this_method_u8s
                            .by_ref()
                            .map(|ch| char::from(ch))
                            .take(length),
                    );
                    $char_string.extend($random_chars.by_ref().take(64 - length));
                    black_box(
                        $char_string
                            .chars()
                            .take_while(|ch| ch.$new_method())
                            .count(),
                    )
                })
            },
        );

        $c.bench_function(
            concat!("old_", stringify!($name), "_count_all_char_matches"),
            |b| {
                b.iter(|| {
                    let length = ($random_u8s.next().unwrap() % 65) as usize;
                    $char_string.clear();
                    $char_string.extend(
                        $this_method_u8s
                            .by_ref()
                            .map(|ch| char::from(ch))
                            .take(length),
                    );
                    $char_string.extend($random_chars.by_ref().take(64 - length));
                    black_box($char_string.chars().filter(|ch| ch.$old_method()).count())
                })
            },
        );

        $c.bench_function(
            concat!("new_", stringify!($name), "_count_all_char_matches"),
            |b| {
                b.iter(|| {
                    let length = ($random_u8s.next().unwrap() % 65) as usize;
                    $char_string.clear();
                    $char_string.extend(
                        $this_method_u8s
                            .by_ref()
                            .map(|ch| char::from(ch))
                            .take(length),
                    );
                    $char_string.extend($random_chars.by_ref().take(64 - length));
                    black_box($char_string.chars().filter(|ch| ch.$new_method()).count())
                })
            },
        );
    };
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut random_u8s = thread_rng().sample_iter::<u8, Standard>(Standard);
    let mut random_chars = thread_rng()
        .sample_iter::<u32, Standard>(Standard)
        .map(|mut ch| {
            ch %= 0x0010_f800;
            ch += if ch < 0x0000_d800 { 0 } else { 0x0000_0800 };
            unsafe { char::from_u32_unchecked(ch) }
        });

    let mut ascii_u8s = thread_rng()
        .sample_iter::<u8, Standard>(Standard)
        .map(|ch| ch & 0b0111_1111);

    let mut alphabetic_u8s = thread_rng()
        .sample_iter::<u8, Standard>(Standard)
        .map(|mut ch| {
            ch %= 52;
            ch += if ch < 26 { b'A' } else { b'a' - 26 };
            ch
        });

    let mut alphanumeric_u8s = thread_rng()
        .sample_iter::<u8, Standard>(Standard)
        .map(|mut ch| {
            ch %= 62;
            ch += if ch < 10 {
                b'0'
            } else if ch < 36 {
                b'A' - 10
            } else {
                b'a' - 36
            };
            ch
        });

    let mut control_u8s = thread_rng()
        .sample_iter::<u8, Standard>(Standard)
        .map(|mut ch| {
            ch %= 33;
            ch += if ch < b' ' { 0 } else { 127 - 32 };
            ch
        });

    let mut digit_u8s = thread_rng()
        .sample_iter::<u8, Standard>(Standard)
        .map(|mut ch| {
            ch %= 10;
            ch += b'0';
            ch
        });

    let mut graphic_u8s = thread_rng()
        .sample_iter::<u8, Standard>(Standard)
        .map(|mut ch| {
            ch %= b'~' - b'!' + 1;
            ch += b'!';
            ch
        });

    let mut hexdigit_u8s = thread_rng()
        .sample_iter::<u8, Standard>(Standard)
        .map(|mut ch| {
            ch %= 22;
            ch += if ch < 10 {
                b'0'
            } else if ch < 16 {
                b'A' - 10
            } else {
                b'a' - 16
            };
            ch
        });

    let mut lowercase_u8s = thread_rng()
        .sample_iter::<u8, Standard>(Standard)
        .map(|mut ch| {
            ch %= 26;
            ch += b'a';
            ch
        });

    let mut punctuation_u8s = thread_rng()
        .sample_iter::<u8, Standard>(Standard)
        .map(|mut ch| {
            ch &= 31;
            ch += if ch < 15 {
                b'!'
            } else if ch < 22 {
                b':' - 15
            } else if ch < 28 {
                b'[' - 22
            } else {
                b'{' - 28
            };
            ch
        });

    let mut uppercase_u8s = thread_rng()
        .sample_iter::<u8, Standard>(Standard)
        .map(|mut ch| {
            ch %= 26;
            ch += b'A';
            ch
        });

    let mut whitespace_u8s = thread_rng()
        .sample_iter::<u8, Standard>(Standard)
        .map(|mut ch| {
            ch %= 5;
            ch += if ch < 2 {
                b'\t'
            } else if ch < 4 {
                b'\x0C' - 2
            } else {
                b' ' - 4
            };
            ch
        });

    let mut u8_string: Vec<u8> = Vec::with_capacity(64);
    let mut char_string: String = String::with_capacity(256);

    ascii_benches!(
        ascii,
        ascii_u8s,
        is_ascii,
        is_ascii_2,
        c,
        ascii_u8s,
        random_u8s,
        random_chars,
        u8_string,
        char_string,
    );

    ascii_benches!(
        alphabetic,
        alphabetic_u8s,
        is_ascii_alphabetic,
        is_ascii_alphabetic_2,
        c,
        ascii_u8s,
        random_u8s,
        random_chars,
        u8_string,
        char_string,
    );

    ascii_benches!(
        alphanumeric,
        alphanumeric_u8s,
        is_ascii_alphanumeric,
        is_ascii_alphanumeric_2,
        c,
        ascii_u8s,
        random_u8s,
        random_chars,
        u8_string,
        char_string,
    );

    ascii_benches!(
        control,
        control_u8s,
        is_ascii_control,
        is_ascii_control_2,
        c,
        ascii_u8s,
        random_u8s,
        random_chars,
        u8_string,
        char_string,
    );

    ascii_benches!(
        digit,
        digit_u8s,
        is_ascii_digit,
        is_ascii_digit_2,
        c,
        ascii_u8s,
        random_u8s,
        random_chars,
        u8_string,
        char_string,
    );

    ascii_benches!(
        graphic,
        graphic_u8s,
        is_ascii_graphic,
        is_ascii_graphic_2,
        c,
        ascii_u8s,
        random_u8s,
        random_chars,
        u8_string,
        char_string,
    );

    ascii_benches!(
        hexdigit,
        hexdigit_u8s,
        is_ascii_hexdigit,
        is_ascii_hexdigit_2,
        c,
        ascii_u8s,
        random_u8s,
        random_chars,
        u8_string,
        char_string,
    );

    ascii_benches!(
        lowercase,
        lowercase_u8s,
        is_ascii_lowercase,
        is_ascii_lowercase_2,
        c,
        ascii_u8s,
        random_u8s,
        random_chars,
        u8_string,
        char_string,
    );

    ascii_benches!(
        punctuation,
        punctuation_u8s,
        is_ascii_punctuation,
        is_ascii_punctuation_2,
        c,
        ascii_u8s,
        random_u8s,
        random_chars,
        u8_string,
        char_string,
    );

    ascii_benches!(
        uppercase,
        uppercase_u8s,
        is_ascii_uppercase,
        is_ascii_uppercase_2,
        c,
        ascii_u8s,
        random_u8s,
        random_chars,
        u8_string,
        char_string,
    );

    ascii_benches!(
        whitespace,
        whitespace_u8s,
        is_ascii_whitespace,
        is_ascii_whitespace_2,
        c,
        ascii_u8s,
        random_u8s,
        random_chars,
        u8_string,
        char_string,
    );
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark
}
criterion_main!(benches);
