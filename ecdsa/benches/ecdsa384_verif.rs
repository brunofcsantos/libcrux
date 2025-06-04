use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use rand::Rng;
use std::time::Duration;

use libcrux_p256::{ecdsa_sign_p256_sha384, ecdsa_verif_p256_sha384};


mod cycles;
use cycles::CpuCycles;

fn bench_ecdsa_verify_cycles(c: &mut Criterion<CpuCycles>) {
    let mut rng = rand::thread_rng();

    let private_key: [u8; 32] = rng.gen();
    let nonce: [u8; 32] = rng.gen();
    let message: Vec<u8> = (0..64).map(|_| rng.gen()).collect();
    let msg_len = message.len() as u32;

    let mut signature = vec![0u8; 64];
    ecdsa_sign_p256_sha384(
        &mut signature,
        msg_len,
        &message,
        &private_key,
        &nonce,
    );

    let (signature_r, signature_s) = signature.split_at(32);

    let mut public_key = [0u8; 64];
    rng.fill(&mut public_key);

    let mut group = c.benchmark_group("ecdsa_verif_p256_sha384");
    group.throughput(Throughput::Bytes(msg_len as u64));
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("ecdsa_verif_p256_sha384", |b| {
        b.iter(|| {
            let _valid = ecdsa_verif_p256_sha384(
                black_box(msg_len),
                black_box(&message),
                black_box(&public_key),
                black_box(signature_r),
                black_box(signature_s),
            );

        });
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_measurement(CpuCycles);
    targets = bench_ecdsa_verify_cycles
}
criterion_main!(benches);
