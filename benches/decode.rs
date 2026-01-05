mod common;

use common::{all_configs, generate_data, BenchConfig};
use divan::{black_box, Bencher};

fn main() {
    divan::main();
}

// ============================================================================
// reed-solomon-erasure benchmarks
// ============================================================================

#[divan::bench_group(name = "decode_erasure")]
mod decode_erasure {
    use super::*;
    use reed_solomon_erasure::galois_8::ReedSolomon;

    fn bench_config(bencher: Bencher, config: BenchConfig, missing_count: usize) {
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

        // Create a scenario with missing shards
        let mut shards_with_missing = shards.clone();
        for i in 0..missing_count {
            shards_with_missing[i] = vec![0u8; shard_size];
        }

        bencher.bench_local(|| {
            let mut shards_clone: Vec<Option<Vec<u8>>> = shards_with_missing
                .iter()
                .enumerate()
                .map(|(i, shard)| {
                    if i < missing_count {
                        None
                    } else {
                        Some(shard.clone())
                    }
                })
                .collect();

            encoder.reconstruct(&mut shards_clone).unwrap();
            black_box(shards_clone);
        });
    }

    #[divan::bench(args = all_configs())]
    fn decode_1_missing(bencher: Bencher, config: BenchConfig) {
        bench_config(bencher, config, 1);
    }

    #[divan::bench(args = all_configs())]
    fn decode_f_missing(bencher: Bencher, config: BenchConfig) {
        bench_config(bencher, config, config.f);
    }

    #[divan::bench(args = all_configs())]
    fn decode_2f_missing(bencher: Bencher, config: BenchConfig) {
        bench_config(bencher, config, 2 * config.f);
    }
}

#[divan::bench_group(name = "decode_novelpoly")]
mod decode_novelpoly {
    use super::*;
    use reed_solomon_novelpoly::{CodeParams, WrappedShard};

    fn bench_config(bencher: Bencher, config: BenchConfig, missing_count: usize) {
        let data = generate_data(config.data_size);
        let total_shards = config.total_shards();
        let data_shards = config.data_shards();

        // Encode data
        let params = CodeParams::derive_parameters(total_shards, data_shards).unwrap();
        let encoder = params.make_encoder();
        let shards: Vec<WrappedShard> = encoder.encode(&data).unwrap();

        // Create missing shards scenario
        let shards_with_missing: Vec<Option<WrappedShard>> = shards
            .into_iter()
            .enumerate()
            .map(
                |(i, shard)| {
                    if i < missing_count {
                        None
                    } else {
                        Some(shard)
                    }
                },
            )
            .collect();

        bencher.bench_local(|| {
            let recovered =
                reed_solomon_novelpoly::reconstruct(shards_with_missing.clone(), total_shards)
                    .unwrap();
            black_box(recovered);
        });
    }

    #[divan::bench(args = all_configs())]
    fn decode_1_missing(bencher: Bencher, config: BenchConfig) {
        bench_config(bencher, config, 1);
    }

    #[divan::bench(args = all_configs())]
    fn decode_f_missing(bencher: Bencher, config: BenchConfig) {
        bench_config(bencher, config, config.f);
    }

    #[divan::bench(args = all_configs())]
    fn decode_2f_missing(bencher: Bencher, config: BenchConfig) {
        bench_config(bencher, config, 2 * config.f);
    }
}

// ============================================================================
// reed-solomon-16 benchmarks
// ============================================================================

#[divan::bench_group(name = "decode_rs16")]
mod decode_rs16 {
    use super::*;
    use reed_solomon_16::ReedSolomonDecoder;

    fn bench_config(bencher: Bencher, config: BenchConfig, missing_count: usize) {
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

        let recovery_provided_count = missing_count;

        bencher.bench_local(|| {
            let mut decoder =
                ReedSolomonDecoder::new(config.data_shards(), config.coding_shards(), shard_bytes)
                    .unwrap();

            // Add available original shards
            for index in missing_count..config.data_shards() {
                decoder
                    .add_original_shard(index, &original_shards[index])
                    .unwrap();
            }

            // Add recovery shards to compensate for missing originals
            for index in 0..recovery_provided_count {
                decoder.add_recovery_shard(index, &recovery[index]).unwrap();
            }

            let result = decoder.decode().unwrap();
            black_box(result);
        });
    }

    #[divan::bench(args = all_configs())]
    fn decode_1_missing(bencher: Bencher, config: BenchConfig) {
        bench_config(bencher, config, 1);
    }

    #[divan::bench(args = all_configs())]
    fn decode_f_missing(bencher: Bencher, config: BenchConfig) {
        bench_config(bencher, config, config.f);
    }

    #[divan::bench(args = all_configs())]
    fn decode_2f_missing(bencher: Bencher, config: BenchConfig) {
        bench_config(bencher, config, 2 * config.f);
    }
}

// ============================================================================
// reed-solomon-simd benchmarks
// ============================================================================

#[divan::bench_group(name = "decode_simd")]
mod decode_simd {
    use super::*;
    use reed_solomon_simd::ReedSolomonDecoder;

    fn bench_config(bencher: Bencher, config: BenchConfig, missing_count: usize) {
        let data = generate_data(config.data_size);
        let shard_bytes = config.shard_size(); // Use aligned size for simd

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
        let recovery = reed_solomon_simd::encode(
            config.data_shards(),
            config.coding_shards(),
            &original_shards,
        )
        .unwrap();

        let recovery_provided_count = missing_count;

        bencher.bench_local(|| {
            let mut decoder =
                ReedSolomonDecoder::new(config.data_shards(), config.coding_shards(), shard_bytes)
                    .unwrap();

            // Add available original shards
            for index in missing_count..config.data_shards() {
                decoder
                    .add_original_shard(index, &original_shards[index])
                    .unwrap();
            }

            // Add recovery shards to compensate for missing originals
            for index in 0..recovery_provided_count {
                decoder.add_recovery_shard(index, &recovery[index]).unwrap();
            }

            let result = decoder.decode().unwrap();
            black_box(result);
        });
    }

    #[divan::bench(args = all_configs())]
    fn decode_1_missing(bencher: Bencher, config: BenchConfig) {
        bench_config(bencher, config, 1);
    }

    #[divan::bench(args = all_configs())]
    fn decode_f_missing(bencher: Bencher, config: BenchConfig) {
        bench_config(bencher, config, config.f);
    }

    #[divan::bench(args = all_configs())]
    fn decode_2f_missing(bencher: Bencher, config: BenchConfig) {
        bench_config(bencher, config, 2 * config.f);
    }
}
