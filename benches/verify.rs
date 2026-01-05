mod common;

use common::{all_configs, generate_data, BenchConfig};
use divan::{black_box, Bencher};

fn main() {
    divan::main();
}

// ============================================================================
// reed-solomon-erasure benchmarks
// ============================================================================

#[divan::bench_group(name = "verify_erasure")]
mod verify_erasure {
    use super::*;
    use reed_solomon_erasure::galois_8::ReedSolomon;

    fn bench_config(bencher: Bencher, config: BenchConfig) {
        let data = generate_data(config.data_size);
        let shard_size = config.shard_size();

        // Prepare and encode shards
        let encoder = ReedSolomon::new(config.data_shards(), config.coding_shards()).unwrap();

        let mut shards: Vec<Vec<u8>> = Vec::new();
        for i in 0..config.data_shards() {
            let start = i * shard_size;
            let end = std::cmp::min(start + shard_size, data.len());
            let mut shard = data[start..end].to_vec();
            shard.resize(shard_size, 0);
            shards.push(shard);
        }

        for _ in 0..config.coding_shards() {
            shards.push(vec![0u8; shard_size]);
        }

        encoder.encode(&mut shards).unwrap();

        bencher.bench_local(|| {
            let result = encoder.verify(&shards).unwrap();
            black_box(result);
        });
    }

    #[divan::bench(args = all_configs())]
    fn verify(bencher: Bencher, config: BenchConfig) {
        bench_config(bencher, config);
    }
}

// ============================================================================
// reed-solomon-simd benchmarks
// ============================================================================

#[divan::bench_group(name = "verify_novelpoly")]
mod verify_novelpoly {
    use super::*;
    use reed_solomon_novelpoly::{CodeParams, WrappedShard};

    fn bench_config(bencher: Bencher, config: BenchConfig) {
        let data = generate_data(config.data_size);
        let total_shards = config.total_shards();
        let data_shards = config.data_shards();

        // Encode data
        let params = CodeParams::derive_parameters(total_shards, data_shards).unwrap();
        let encoder = params.make_encoder();
        let _shards: Vec<WrappedShard> = encoder.encode(&data).unwrap();

        bencher.bench_local(|| {
            // Verify by re-encoding
            let verify_shards: Vec<WrappedShard> = encoder.encode(&data).unwrap();
            black_box(verify_shards);
        });
    }

    #[divan::bench(args = all_configs())]
    fn verify(bencher: Bencher, config: BenchConfig) {
        bench_config(bencher, config);
    }
}

// ============================================================================
// reed-solomon-16 benchmarks
// ============================================================================

#[divan::bench_group(name = "verify_rs16")]
mod verify_rs16 {
    use super::*;

    fn bench_config(bencher: Bencher, config: BenchConfig) {
        let data = generate_data(config.data_size);
        let shard_bytes = config.shard_size(); // Use aligned size for rs16

        // Split data into shards
        let mut original_shards = Vec::new();
        for i in 0..config.data_shards() {
            let start = i * config.shard_size();
            let end = std::cmp::min(start + config.shard_size(), data.len());
            let mut shard = data[start..end].to_vec();
            shard.resize(shard_bytes, 0);
            original_shards.push(shard);
        }

        // Encode data
        let recovery = reed_solomon_16::encode(
            config.data_shards(),
            config.coding_shards(),
            &original_shards,
        )
        .unwrap();

        bencher.bench_local(|| {
            // Verify by re-encoding and comparing
            let verify_recovery = reed_solomon_16::encode(
                config.data_shards(),
                config.coding_shards(),
                &original_shards,
            )
            .unwrap();
            let is_valid = verify_recovery == recovery;
            black_box(is_valid);
        });
    }

    #[divan::bench(args = all_configs())]
    fn verify(bencher: Bencher, config: BenchConfig) {
        bench_config(bencher, config);
    }
}

// ============================================================================
// reed-solomon-simd benchmarks
// ============================================================================

#[divan::bench_group(name = "verify_simd")]
mod verify_simd {
    use super::*;

    fn bench_config(bencher: Bencher, config: BenchConfig) {
        let data = generate_data(config.data_size);
        let shard_bytes = config.shard_size(); // Use aligned size for simd

        // Encode data
        let recovery = reed_solomon_simd::encode(
            config.data_shards(),
            config.coding_shards(),
            &vec![data.clone()],
        )
        .unwrap();

        // Split data into shards
        let mut original_shards = Vec::new();
        for i in 0..config.data_shards() {
            let start = i * config.shard_size();
            let end = std::cmp::min(start + config.shard_size(), data.len());
            let mut shard = data[start..end].to_vec();
            shard.resize(shard_bytes, 0);
            original_shards.push(shard);
        }

        bencher.bench_local(|| {
            // Verify by re-encoding and comparing
            let verify_recovery = reed_solomon_simd::encode(
                config.data_shards(),
                config.coding_shards(),
                &vec![data.clone()],
            )
            .unwrap();
            let is_valid = verify_recovery == recovery;
            black_box(is_valid);
        });
    }

    #[divan::bench(args = all_configs())]
    fn verify(bencher: Bencher, config: BenchConfig) {
        bench_config(bencher, config);
    }
}
