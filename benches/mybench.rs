use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vsmp_bot::message_handler::{get_response_1, get_response_2, get_response_3};
use vsmp_bot::message_parser::get_quick_message;

fn bench_functions(c: &mut Criterion) {
    let text = "hola mundo rust hola foo bar adssj qrdbuisf oisdnf qkqdsa Jewish burger indelible crazy rubber room rubber rats";
    let quick_message = get_quick_message(&text);

    c.bench_function("Original", |b| {
        b.iter(|| get_response_1(black_box(&text.to_string())))
    });

    c.bench_function("Quick message + tokenize", |b| {
        b.iter(|| get_response_2(black_box(&text.to_string())))
    });

    c.bench_function("Quick message ALONE", |b| {
        b.iter(|| get_response_3(black_box(&quick_message)))
    });
}

criterion_group!(benches, bench_functions);
criterion_main!(benches);