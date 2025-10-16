// benches/primitives.rs

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use sha3::{Digest, Keccak256};
use std::env;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

struct Args {
    threads: usize,
    hashes: usize,
}

/// A helper that returns a vector of random 32-byte arrays, one per hash call.
fn make_random_inputs(count: usize) -> Vec<[u8; 32]> {
    // Use a deterministic seed for reproducible benchmarks
    let mut rng = StdRng::from_os_rng();
    let mut buf = vec![[0u8; 32]; count];
    for x in &mut buf {
        rng.fill_bytes(x);
    }
    buf
}

/// Run the benchmarked workload: multiple threads hashing random inputs
fn keccak_hash_random(num_threads: usize, hashes_per_thread: usize) -> f64 {
    // Pre-generate all random inputs for each thread
    // We generate thread_count * hashes_per_thread inputs.
    let total = num_threads
        .checked_mul(hashes_per_thread)
        .expect("overflow in product");
    let all_inputs = make_random_inputs(total);
    let all_inputs = Arc::new(all_inputs);

    let start = Instant::now();

    let mut handles = Vec::with_capacity(num_threads);
    for thread_idx in 0..num_threads {
        let inputs = Arc::clone(&all_inputs);
        handles.push(thread::spawn(move || {
            // Each thread processes a disjoint slice of the inputs vector
            let offset = thread_idx * hashes_per_thread;
            let slice = &inputs[offset..offset + hashes_per_thread];

            for inp in slice {
                let mut hasher = Keccak256::new();
                hasher.update(black_box(inp));
                let _ = hasher.finalize();
            }
        }));
    }

    for h in handles {
        h.join().expect("thread panicked");
    }

    let elapsed = start.elapsed().as_secs_f64();
    let total_hashes = (num_threads * hashes_per_thread) as f64;
    total_hashes / elapsed
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let threads_env = env::var("THREADS").unwrap_or_else(|_| String::from("1"));
    let threads: usize = threads_env.parse().unwrap();
    let hashes_env = env::var("HASHES").unwrap_or_else(|_| String::from("100000"));
    let hashes: usize = hashes_env.parse().unwrap();
    println!("Using THREADS env var: {threads}");

    let args = Args {
        threads: threads,
        hashes: hashes,
    };
    let bench_id = format!("keccak256_random_{}threads", args.threads);
    c.bench_function(&bench_id, |b| {
        b.iter(|| {
            let hps = keccak_hash_random(args.threads, args.hashes);
            black_box(hps);
            println!(
                "Throughput  with {} threads × {} hashes..: {:.2} hashes/sec",
                args.threads, args.hashes, hps
            );
        });
    });
}

// THREADS=4 HASHES=100000  cargo bench --bench hash

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
