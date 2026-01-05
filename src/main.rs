fn main() {
    println!("Reed-Solomon Benchmark Suite");
    println!("Run benchmarks with: cargo bench");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_reed_solomon_erasure() {
        use reed_solomon_erasure::galois_8::ReedSolomon;

        let encoder = ReedSolomon::new(3, 2).unwrap();

        let mut shards: Vec<Vec<u8>> =
            vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![0, 0], vec![0, 0]];

        encoder.encode(&mut shards).unwrap();
        println!("reed-solomon-erasure encode works");
    }

    #[test]
    fn test_reed_solomon_novelpoly() {
        use reed_solomon_novelpoly::{CodeParams, WrappedShard};

        let params = CodeParams::derive_parameters(5, 3).unwrap();
        let encoder = params.make_encoder();
        let data = vec![1u8, 2, 3, 4, 5, 6];
        let _shards: Vec<WrappedShard> = encoder.encode(&data).unwrap();
        println!("reed-solomon-novelpoly encode works");
    }

    #[test]
    fn test_reed_solomon_16() {
        use reed_solomon_16::ReedSolomonEncoder;

        let shard_bytes = 64; // Minimum shard size
        let mut encoder = ReedSolomonEncoder::new(3, 2, shard_bytes).unwrap();

        encoder.add_original_shard(&vec![1u8; shard_bytes]).unwrap();
        encoder.add_original_shard(&vec![2u8; shard_bytes]).unwrap();
        encoder.add_original_shard(&vec![3u8; shard_bytes]).unwrap();

        let _parity = encoder.encode().unwrap();
        println!("reed-solomon-16 encode works");
    }
}
