mod common;

use common::{all_configs, generate_data, BenchConfig};
use divan::{black_box, Bencher};

fn main() {
    divan::main();
}

// ============================================================================
// reed-solomon-erasure benchmarks
// ============================================================================

#[divan::bench_group(name = "encode_erasure")]
mod encode_erasure {
    use super::*;
    use reed_solomon_erasure::galois_8::ReedSolomon;

    fn bench_config(bencher: Bencher, config: BenchConfig) {
        let data = generate_data(config.data_size);
        let shard_size = config.shard_size();

        // Prepare data shards
        let mut shards: Vec<Vec<u8>> = Vec::new();
        for i in 0..config.data_shards() {
            let start = i * shard_size;
            let end = std::cmp::min(start + shard_size, data.len());
            let mut shard = data[start..end].to_vec();
            shard.resize(shard_size, 0);
            shards.push(shard);
        }

        // Add empty parity shards
        for _ in 0..config.coding_shards() {
            shards.push(vec![0u8; shard_size]);
        }

        bencher.bench_local(|| {
            let encoder = ReedSolomon::new(config.data_shards(), config.coding_shards()).unwrap();

            let mut shards_clone = shards.clone();
            encoder.encode(&mut shards_clone).unwrap();
            black_box(shards_clone);
        });
    }

    #[divan::bench(args = all_configs())]
    fn encode(bencher: Bencher, config: BenchConfig) {
        bench_config(bencher, config);
    }
}

// ============================================================================
// reed-solomon-novelpoly benchmarks
// ============================================================================

#[divan::bench_group(name = "encode_novelpoly")]
mod encode_novelpoly {
    use super::*;
    use reed_solomon_novelpoly::{CodeParams, WrappedShard};

    fn bench_config(bencher: Bencher, config: BenchConfig) {
        let data = generate_data(config.data_size);
        let total_shards = config.total_shards();
        let data_shards = config.data_shards();

        bencher.bench_local(|| {
            // novelpoly uses n (total) and k (data) parameters
            let params = CodeParams::derive_parameters(total_shards, data_shards).unwrap();
            let encoder = params.make_encoder();
            let shards: Vec<WrappedShard> = encoder.encode(&data).unwrap();
            black_box(shards);
        });
    }

    #[divan::bench(args = all_configs())]
    fn encode(bencher: Bencher, config: BenchConfig) {
        bench_config(bencher, config);
    }
}

// ============================================================================
// reed-solomon-16 benchmarks
// ============================================================================

#[divan::bench_group(name = "encode_rs16")]
mod encode_rs16 {
    use super::*;
    use reed_solomon_16::ReedSolomonEncoder;

    fn bench_config(bencher: Bencher, config: BenchConfig) {
        let data = generate_data(config.data_size);
        let shard_bytes = config.shard_size();

        bencher.bench_local(|| {
            let mut encoder =
                ReedSolomonEncoder::new(config.data_shards(), config.coding_shards(), shard_bytes)
                    .unwrap();

            // Add data shards
            for i in 0..config.data_shards() {
                let start = i * shard_bytes;
                let end = std::cmp::min(start + shard_bytes, data.len());
                let mut shard = data[start..end].to_vec();
                shard.resize(shard_bytes, 0);
                encoder.add_original_shard(&shard).unwrap();
            }

            let result = encoder.encode().unwrap();
            black_box(result);
        });
    }

    #[divan::bench(args = all_configs())]
    fn encode(bencher: Bencher, config: BenchConfig) {
        bench_config(bencher, config);
    }
}

// ============================================================================
// reed-solomon-simd benchmarks
// ============================================================================

#[divan::bench_group(name = "encode_simd")]
mod encode_simd {
    use super::*;
    use reed_solomon_simd::ReedSolomonEncoder;

    fn bench_config(bencher: Bencher, config: BenchConfig) {
        let data = generate_data(config.data_size);
        let shard_bytes = config.shard_size();

        bencher.bench_local(|| {
            let mut encoder =
                ReedSolomonEncoder::new(config.data_shards(), config.coding_shards(), shard_bytes)
                    .unwrap();

            // Add data shards
            for i in 0..config.data_shards() {
                let start = i * shard_bytes;
                let end = std::cmp::min(start + shard_bytes, data.len());
                let mut shard = data[start..end].to_vec();
                shard.resize(shard_bytes, 0);
                encoder.add_original_shard(&shard).unwrap();
            }

            let result = encoder.encode().unwrap();
            black_box(result);
        });
    }

    #[divan::bench(args = all_configs())]
    fn encode(bencher: Bencher, config: BenchConfig) {
        bench_config(bencher, config);
    }
}
