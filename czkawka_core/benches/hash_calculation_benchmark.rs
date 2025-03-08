use std::env::temp_dir;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use czkawka_core::tools::duplicate::{DuplicateEntry, HashType, hash_calculation};

fn setup_test_file(size: u64) -> PathBuf {
    let mut path = temp_dir();
    path.push("test_file");
    let mut file = File::create(&path).expect("Failed to create test file");
    file.write_all(&vec![0u8; size as usize]).expect("Failed to write to test file");
    path
}

fn get_file_entry(size: u64) -> DuplicateEntry {
    let path = setup_test_file(size);
    DuplicateEntry {
        path,
        modified_date: 0,
        size,
        hash: String::new(),
    }
}

fn benchmark_hash_calculation_vec<const FILE_SIZE: u64, const BUFFER_SIZE: usize>(c: &mut Criterion) {
    let file_entry = get_file_entry(FILE_SIZE);
    let function_name = format!("hash_calculation_vec_file_{FILE_SIZE}_buffer_{BUFFER_SIZE}");

    c.bench_function(&function_name, |b| {
        b.iter(|| {
            let mut buffer = vec![0u8; BUFFER_SIZE];
            hash_calculation(black_box(&mut buffer), black_box(&file_entry), black_box(HashType::Blake3), &Arc::default(), None).expect("Failed to calculate hash");
        });
    });
}

fn benchmark_hash_calculation_arr<const FILE_SIZE: u64, const BUFFER_SIZE: usize>(c: &mut Criterion) {
    let file_entry = get_file_entry(FILE_SIZE);
    let function_name = format!("hash_calculation_arr_file_{FILE_SIZE}_buffer_{BUFFER_SIZE}");

    c.bench_function(&function_name, |b| {
        b.iter(|| {
            let mut buffer = [0u8; BUFFER_SIZE];
            hash_calculation(black_box(&mut buffer), black_box(&file_entry), black_box(HashType::Blake3), &Arc::default(), None).expect("Failed to calculate hash");
        });
    });
}

criterion_group!(benches,
    benchmark_hash_calculation_vec<{16 * 1024 * 1024}, {16 * 1024}>,
    benchmark_hash_calculation_vec<{16 * 1024 * 1024}, {1024 * 1024}>,
    benchmark_hash_calculation_arr<{16 * 1024 * 1024}, {16 * 1024}>,
    benchmark_hash_calculation_arr<{16 * 1024 * 1024}, {1024 * 1024}>,
);
criterion_main!(benches);
