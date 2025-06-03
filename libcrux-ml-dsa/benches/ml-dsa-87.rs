use criterion::{criterion_group, criterion_main, Criterion};
use rand::{rngs::OsRng, TryRngCore};
use libcrux_ml_dsa::ml_dsa_87;
use std::time::Duration;

use nix::sched::{sched_setaffinity, CpuSet};
use nix::unistd::Pid;

mod cycles;
use cycles::CpuCycles;

pub fn comparisons_key_generation(c: &mut Criterion<CpuCycles>) {
    let mut rng = OsRng;
    let mut group = c.benchmark_group("ML-DSA-87 Key Generation");
    group.measurement_time(Duration::from_secs(10));

    let mut randomness = [0u8; 32];
    rng.try_fill_bytes(&mut randomness).unwrap();

    group.bench_function("libcrux (external random)", move |b| {
        b.iter(|| {
            let _ = ml_dsa_87::generate_key_pair(randomness);
        });
    });

    #[cfg(not(all(target_os = "macos", target_arch = "x86_64")))]
    group.bench_function("pqclean (internal random)", |b| {
        b.iter(|| {
            let _ = pqcrypto_mldsa::mldsa87::keypair();
        });
    });

    group.finish();
}

pub fn comparisons_signing(c: &mut Criterion<CpuCycles>) {
    let mut rng = OsRng;
    let mut group = c.benchmark_group("ML-DSA-87 Signing");
    group.measurement_time(Duration::from_secs(10));

    let mut message = [0u8; 511];
    rng.try_fill_bytes(&mut message).unwrap();

    let mut randomness = [0; 32];
    rng.try_fill_bytes(&mut randomness).unwrap();
    let keypair = ml_dsa_87::generate_key_pair(randomness);

    rng.try_fill_bytes(&mut randomness).unwrap();

    group.bench_function("libcrux (external random)", move |b| {
        b.iter(|| {
            let _ = ml_dsa_87::sign(&keypair.signing_key, &message, b"", randomness);
        })
    });

    #[cfg(not(all(target_os = "macos", target_arch = "x86_64")))]
    {
        let (_, sk) = pqcrypto_mldsa::mldsa87::keypair();
        group.bench_function("pqclean (internal random)", move |b| {
            b.iter(|| {
                let _ = pqcrypto_mldsa::mldsa87::detached_sign(&message, &sk);
            })
        });
    }
}

pub fn comparisons_verification(c: &mut Criterion<CpuCycles>) {
    let mut rng = OsRng;
    let mut group = c.benchmark_group("ML-DSA-87 Verification");
    group.measurement_time(Duration::from_secs(10));

    let mut message = [0u8; 511];
    rng.try_fill_bytes(&mut message).unwrap();

    let mut randomness = [0; 32];
    rng.try_fill_bytes(&mut randomness).unwrap();
    let keypair = ml_dsa_87::generate_key_pair(randomness);

    rng.try_fill_bytes(&mut randomness).unwrap();
    let signature = ml_dsa_87::sign(&keypair.signing_key, &message, b"", randomness).unwrap();

    group.bench_function("libcrux", move |b| {
        b.iter(|| {
            let _ =
                ml_dsa_87::verify(&keypair.verification_key, &message, b"", &signature).unwrap();
        })
    });

    #[cfg(not(all(target_os = "macos", target_arch = "x86_64")))]
    {
        let (vk, sk) = pqcrypto_mldsa::mldsa87::keypair();
        let signature = pqcrypto_mldsa::mldsa87::detached_sign(&message, &sk);

        group.bench_function("pqclean", move |b| {
            b.iter(|| {
                let _ =
                    pqcrypto_mldsa::mldsa87::verify_detached_signature(&signature, &message, &vk)
                        .unwrap();
            })
        });
    }
}

pub fn comparisons(c: &mut Criterion<CpuCycles>) {
    let num_cpus = unsafe {
        let n = nix::libc::sysconf(nix::libc::_SC_NPROCESSORS_ONLN);
        if n < 1 {
            panic!("Failed to get number of CPUs");
        }
        n as usize
    };

    // Set affinity to the last CPU
    let mut cpuset = CpuSet::new();
    cpuset.set(num_cpus - 1).expect("Failed to set CPU in cpuset");

    sched_setaffinity(Pid::from_raw(0), &cpuset).expect("Failed to set CPU affinity");

    comparisons_key_generation(c);
    comparisons_signing(c);
    comparisons_verification(c);
}


criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(CpuCycles);
    targets = comparisons
);
criterion_main!(benches);
