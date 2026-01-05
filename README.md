# Reed-Solomon Benchmark Suite

A comprehensive benchmark comparing the performance of popular Rust Reed-Solomon erasure coding crates.

## Benchmarked Crates

- **reed-solomon-erasure** (v6.0) - Most widely used, mature implementation
- **reed-solomon-novelpoly** (v2.0) - Performance-focused with novel polynomial basis
- **reed-solomon-16** (v0.1) - Optimized for 16-bit operations

## Test Configuration

### Shard Configurations (F data + 2F coding)

The benchmarks test with the following configurations, where F is the number of data shards:

- F=1: 1 data + 2 coding shards (3 total)
- F=2: 2 data + 4 coding shards (6 total)
- F=3: 3 data + 6 coding shards (9 total)
- F=4: 4 data + 8 coding shards (12 total)
- F=5: 5 data + 10 coding shards (15 total)
- F=10: 10 data + 20 coding shards (30 total)
- F=20: 20 data + 40 coding shards (60 total)
- F=30: 30 data + 60 coding shards (90 total)
- F=33: 33 data + 66 coding shards (99 total)

This provides 2x redundancy - you can lose up to 2F shards and still recover the original data.

### Data Sizes

Each configuration is tested with the following data sizes:

- 1KB
- 4KB
- 16KB
- 64KB
- 256KB
- 1MB
- 4MB
- 16MB
- 64MB
- 100MB

### Operations Benchmarked

1. **Encoding**: Splitting data into shards and generating parity shards
2. **Decoding**: Reconstructing original data from partial shards
   - With 1 missing shard
   - With F missing shards
   - With 2F missing shards (maximum recoverable)
3. **Verification**: Checking data integrity without full decode

## Running the Benchmarks

### Prerequisites

```bash
# Ensure you have Rust installed
rustup update
```

### Run All Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark suite
cargo bench --bench encode
cargo bench --bench decode
cargo bench --bench verify
```

### Run Specific Tests

```bash
# Run only encoding benchmarks for reed-solomon-erasure
cargo bench --bench encode encode_erasure

# Run only decoding benchmarks for reed-solomon-novelpoly
cargo bench --bench decode decode_novelpoly

# Run verification for reed-solomon-16
cargo bench --bench verify verify_rs16
```

### Filter by Configuration

```bash
# Run benchmarks for a specific F value
cargo bench -- F10

# Run benchmarks for a specific data size
cargo bench -- 1MB

# Run benchmarks for specific crate
cargo bench -- erasure
```

## Understanding the Results

Divan outputs detailed statistics for each benchmark:

- **Time**: Mean execution time with standard deviation
- **Throughput**: Data processed per second (for applicable benchmarks)
- **Comparison**: Relative performance between different implementations

### Key Metrics to Consider

1. **Encoding Speed**: How fast can data be split and parity generated?
2. **Decoding Speed**: How fast can missing data be reconstructed?
3. **Scalability**: How does performance change with:
   - Increasing data size
   - Increasing number of shards (F value)
   - Increasing number of missing shards

### Expected Performance Characteristics

- **Small F values (1-5)**: Lower overhead, faster for small data
- **Large F values (20-33)**: Better parallelization potential, more efficient for large data
- **Encoding**: Generally faster than decoding
- **Decoding**: Slower with more missing shards (more computation needed)

## Interpreting Results

### When to Use Each Crate

**reed-solomon-erasure**:
- Most mature and widely tested
- Good all-around performance
- Comprehensive API with verification support

**reed-solomon-novelpoly**:
- Optimized for performance with novel polynomial basis
- May excel at specific configurations
- Good for performance-critical applications

**reed-solomon-16**:
- Uses 16-bit operations instead of 8-bit
- May be faster on certain architectures
- Different memory characteristics

## Hardware Specifications

Record your hardware specifications when running benchmarks:

```bash
# Linux
lscpu
free -h

# macOS
sysctl -n machdep.cpu.brand_string
sysctl hw.memsize

# Windows
wmic cpu get name
wmic computersystem get totalphysicalmemory
```

## Benchmark Results

Results will be saved in `target/criterion/` directory with detailed HTML reports.

## Contributing

To add a new Reed-Solomon crate to the benchmark:

1. Add the crate to `Cargo.toml` dependencies
2. Create a new benchmark group in each of `encode.rs`, `decode.rs`, and `verify.rs`
3. Follow the existing pattern for consistency
4. Update this README with the new crate information

## License

This benchmark suite is provided as-is for performance comparison purposes.

## Notes

- Benchmarks use deterministic data generation for reproducibility
- Each benchmark is run multiple times to ensure statistical significance
- Results may vary based on hardware, OS, and system load
- For production use, always test with your specific use case and data patterns

