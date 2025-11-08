use rayon::prelude::*;
use std::str::FromStr;

use alloy_primitives::{B256, Signature, address, b256};
use criterion::{Criterion, black_box, criterion_group, criterion_main};

#[inline]
fn sender_recovery_seq(sigs: &[(Signature, B256)]) {
    for (sig, hash) in sigs.iter() {
        sig.recover_from_prehash(hash).ok();
    }
}

#[inline]
fn sender_recovery_par(sigs: &[(Signature, B256)]) {
    sigs.par_iter().for_each(|(sig, hash)| {
        sig.recover_from_prehash(hash).ok();
    });
}

pub fn criterion_benchmark_sender_recovery(c: &mut Criterion) {
    let sig = Signature::from_str(
            "b91467e570a6466aa9e9876cbcd013baba02900b8979d43fe208a4a4f339f5fd6007e74cd82e037b800186422fc2da167c747ef045e5d18a5f5d4300f8e1a0291c"
        ).expect("could not parse signature");
    let hash = b256!("0x5eb4f5a33c621f32a8622d5f943b6b102994dfe4e5aebbefe69bb1b2aa0fc93e");
    let expected = address!("0x2c7536E3605D9C16a7a3D7b1898e529396a65c23");
    let n = 100;
    let mut sigs = Vec::with_capacity(n);
    for _ in 0..n {
        sigs.push((sig, hash));
    }

    assert_eq!(sig.recover_address_from_msg("Some data").unwrap(), expected);

    c.bench_function("sequential_sender_recovery", |b| {
        b.iter(|| black_box(sender_recovery_seq(&sigs)))
    });

    c.bench_function("parallel_sender_recovery", |b| {
        b.iter(|| black_box(sender_recovery_par(&sigs)))
    });
}

criterion_group!(benches, criterion_benchmark_sender_recovery);
criterion_main!(benches);

// the excution time is almost linear with cores
// each sender recovery cost about 100 µs, which is far beyond rayon's overhead to start to thread (10~20 µs).
