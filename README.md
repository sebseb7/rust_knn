# rust_knn

[![Rust CI](https://github.com/sebseb7/rust_knn/actions/workflows/rust.yml/badge.svg)](https://github.com/sebseb7/rust_knn/actions/workflows/rust.yml)

A Rust library that provides k-nearest neighbors functionality for string data.

## Features

- Upload string data to in-memory storage
- Search for k-nearest neighbors using Levenshtein distance
- Word-order independent search mode using Jaccard distance
- Fast performance (up to 15x faster in word-order independent mode)

## Usage in Rust

```rust
use rust_knn::{upload_strings, k_nearest_neighbour_sort};

fn main() {
    // Upload strings
    let strings = vec!["apple".to_string(), "banana".to_string(), "orange".to_string()];
    upload_strings(strings).unwrap();
    
    // Find 2 nearest neighbors to "aple" (with word order sensitivity)
    let results = k_nearest_neighbour_sort("aple".to_string(), 2, Some(true)).unwrap();
    println!("Word order sensitive results: {:?}", results);
    
    // Find 2 nearest neighbors using word-order independent search
    let results = k_nearest_neighbour_sort("aple".to_string(), 2, Some(false)).unwrap();
    println!("Word order independent results: {:?}", results);
}
```

## Running the Mock App

The mock app demonstrates both word-order sensitive and word-order independent searches:

```bash
# Regular development build
cargo run --example mock_app

# Optimized release build (much faster)
cargo run --example mock_app --release
```

## Word Order Independence

The library supports two search modes:

1. **Word Order Sensitive** (default): Uses Levenshtein distance to compare strings character by character
   - Good for exact matching or single word queries
   - Order of words matters
   - Example: "premium device techno" and "device premium techno" are considered different

2. **Word Order Independent**: Uses Jaccard distance to compare word sets
   - Better for multi-word queries where word order doesn't matter
   - Much faster (5-20ms vs 20-25ms on 7000+ items)
   - Example: "premium device techno" and "device premium techno" are treated as the same

To use word-order independent search, pass `Some(false)` as the third parameter to `k_nearest_neighbour_sort`.

## Building

```bash
# For Rust only
cargo build --release
```

## Performance

The release build shows impressive performance:

- Word-order sensitive search: ~20-25ms for 7000+ items
- Word-order independent search: ~5-6ms for 7000+ items (4x faster)

## Repository

This project is hosted at [github.com/sebseb7/rust_knn](https://github.com/sebseb7/rust_knn)

## License

Licensed under the [0BSD License](LICENSE) (BSD Zero Clause License) - see the [LICENSE](LICENSE) file for details. 