/// Common utilities for Reed-Solomon benchmarks
use std::collections::HashMap;

/// Configuration for a Reed-Solomon test
#[derive(Debug, Clone, Copy)]
pub struct BenchConfig {
    /// F value (data shards = F, coding shards = 2F)
    pub f: usize,
    /// Data size in bytes
    pub data_size: usize,
}

impl BenchConfig {
    pub fn new(f: usize, data_size: usize) -> Self {
        Self { f, data_size }
    }

    pub fn data_shards(&self) -> usize {
        self.f
    }

    pub fn coding_shards(&self) -> usize {
        2 * self.f
    }

    pub fn total_shards(&self) -> usize {
        3 * self.f
    }

    pub fn shard_size(&self) -> usize {
        (self.data_size + self.f - 1) / self.f
    }
}

/// All F values to test
pub const F_VALUES: &[usize] = &[1, 2, 3, 4, 5, 10, 20, 30, 33];

/// All data sizes to test (in bytes)
pub const DATA_SIZES: &[usize] = &[
    1024,              // 1KB
    4 * 1024,          // 4KB
    16 * 1024,         // 16KB
    64 * 1024,         // 64KB
    256 * 1024,        // 256KB
    1024 * 1024,       // 1MB
    4 * 1024 * 1024,   // 4MB
    16 * 1024 * 1024,  // 16MB
    64 * 1024 * 1024,  // 64MB
    100 * 1024 * 1024, // 100MB
];

/// Generate test data of specified size
pub fn generate_data(size: usize) -> Vec<u8> {
    // Use a simple pattern for reproducibility
    (0..size).map(|i| (i % 256) as u8).collect()
}

/// Format data size for display
#[allow(dead_code)]
pub fn format_size(size: usize) -> String {
    if size >= 1024 * 1024 {
        format!("{}MB", size / (1024 * 1024))
    } else if size >= 1024 {
        format!("{}KB", size / 1024)
    } else {
        format!("{}B", size)
    }
}

/// Generate a benchmark name
#[allow(dead_code)]
pub fn bench_name(crate_name: &str, config: &BenchConfig) -> String {
    format!(
        "{}/F{}/{}",
        crate_name,
        config.f,
        format_size(config.data_size)
    )
}

/// Get all benchmark configurations
pub fn all_configs() -> Vec<BenchConfig> {
    let mut configs = Vec::new();
    for &f in F_VALUES {
        for &data_size in DATA_SIZES {
            configs.push(BenchConfig::new(f, data_size));
        }
    }
    configs
}

/// Cache for pre-generated test data
#[allow(dead_code)]
pub struct DataCache {
    cache: HashMap<usize, Vec<u8>>,
}

#[allow(dead_code)]
impl DataCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn get_or_generate(&mut self, size: usize) -> &Vec<u8> {
        self.cache
            .entry(size)
            .or_insert_with(|| generate_data(size))
    }
}

impl Default for DataCache {
    fn default() -> Self {
        Self::new()
    }
}
