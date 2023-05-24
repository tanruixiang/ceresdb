// Copyright 2022 CeresDB Project Authors. Licensed under Apache-2.0.

// custom hash mod
use std::{collections, time::Instant};

pub use ahash::AHasher;
use byteorder::{ByteOrder, LittleEndian};
use murmur3::murmur3_x64_128;
use rand::Rng;
pub fn hash64(mut bytes: &[u8]) -> u64 {
    let mut out = [0; 16];
    murmur3_x64_128(&mut bytes, 0, &mut out);
    // in most cases we run on little endian target
    LittleEndian::read_u64(&out[0..8])
}

#[cfg(test)]
mod test {
    use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};

    use murmur3::murmur3_32;

    use super::*;

    #[test]
    fn empty_hash_test() {
        let res1 = hash64(&[]);
        let res2 = hash64(&[]);
        assert_eq!(res1, res2);
    }

    #[test]
    fn hash_test() {
        let test_bytes_1 = b"cse_engine_hash_mod_test_bytes1".to_vec();
        let test_bytes_2 = b"cse_engine_hash_mod_test_bytes2".to_vec();
        {
            // hash64 testing
            let res1 = hash64(&test_bytes_1);
            let res1_1 = hash64(&test_bytes_1);
            assert_eq!(res1, res1_1);

            let res2 = hash64(&test_bytes_2);
            assert_ne!(res1, res2);
        }
    }

    #[test]
    fn hash_benchmark() {
        fn generate_random_string(length: usize) -> String {
            let mut rng = rand::thread_rng();
            let chars: Vec<char> = (0..length)
                .map(|_| rng.gen_range(0..36))
                .map(|n| if n < 26 { (n + 97) as u8 } else { (n - 26 + 48) as u8 } as char)
                .collect();
            chars.iter().collect()
        }

        fn test_hash_function<F>(hash_function: F, num_tests: usize, string_length: usize)
        where
            F: Fn(&str) -> u64,
        {
            let mut total_time = 0;
            for _ in 0..num_tests {
                let string_to_hash = generate_random_string(string_length);
                let start_time = Instant::now();
                hash_function(&string_to_hash);
                let end_time = start_time.elapsed();
                total_time += end_time.as_nanos();
            }
            let avg_time = total_time as f64 / num_tests as f64;
            println!(
                "Average time to hash a string of length {}: {} nanoseconds",
                string_length, avg_time
            );
        }
        let a_hash = |x: &str| -> u64 {
            let mut hasher = AHasher::default();
            x.hash(&mut hasher);
            hasher.finish()
        };
        let defalut_hash = |x: &str| -> u64 {
            let mut hasher = DefaultHasher::new();
            x.hash(&mut hasher);
            hasher.finish()
        };
        test_hash_function(defalut_hash, 10000, 10);
        test_hash_function(a_hash, 10000, 10);
        let murmur_hash = |x: &str| -> u64 { hash64(x.as_bytes()) };
        test_hash_function(murmur_hash, 10000, 10)
    }
}
