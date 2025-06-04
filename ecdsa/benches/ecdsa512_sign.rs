use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use rand::Rng;
use std::time::Duration;
use libcrux_p256::ecdsa_sign_p256_sha512;

mod cycles;
use cycles::CpuCycles;

fn bench_ecdsa_sign_cycles(c: &mut Criterion<CpuCycles>) {
    let mut rng = rand::thread_rng();

    let private_key: [u8; 32] = rng.gen();
    let nonce: [u8; 32] = rng.gen();
    let message: Vec<u8> = (0..64).map(|_| rng.gen()).collect();
    let msg_len = message.len() as u32;

    let mut signature = vec![0u8; 64]; // usually 64 bytes for ECDSA P-256 (r + s)

    let mut group = c.benchmark_group("ecdsa_sign_p256_sha512");
    group.throughput(Throughput::Bytes(msg_len as u64));
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("ecdsa_sign_p256_sha512", |b| {
        b.iter(|| {
            let _ = ecdsa_sign_p256_sha512(
                black_box(&mut signature),
                black_box(msg_len),
                black_box(&message),
                black_box(&private_key),
                black_box(&nonce),
            );
        });
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_measurement(CpuCycles);
    targets = bench_ecdsa_sign_cycles
}
criterion_main!(benches);
