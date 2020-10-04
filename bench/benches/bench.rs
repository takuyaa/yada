use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use fnv::FnvHashMap;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;
use yada::builder::DoubleArrayBuilder;
use yada::DoubleArray;

fn bench_build(c: &mut Criterion) {
    let keyset = load_ipadic();

    let mut group = c.benchmark_group("build");
    group.sample_size(20);
    group.warm_up_time(Duration::from_secs(20));
    group.measurement_time(Duration::from_secs(30));
    group.sampling_mode(SamplingMode::Flat);

    group.bench_function("build", |b| {
        b.iter(|| DoubleArrayBuilder::build(keyset.as_slice()));
    });

    group.finish();
}

fn bench_search_sorted(c: &mut Criterion) {
    let keyset_sorted = load_ipadic();
    println!(
        "Start a search benchmark by sorted keys. #keys: {}",
        keyset_sorted.len()
    );

    let mut group = c.benchmark_group("search/sort");
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(5));
    group.sampling_mode(SamplingMode::Flat);

    group.bench_function("BTreeMap", |b| {
        let mut map = BTreeMap::new();
        for (key, value) in keyset_sorted.iter() {
            map.insert(key, value);
        }
        b.iter(|| {
            for (key, _) in keyset_sorted.as_slice() {
                let value = map.get(key);
                if value.is_none() {
                    panic!();
                }
            }
        });
    });
    group.bench_function("HashMap", |b| {
        let mut map = HashMap::new();
        for (key, value) in keyset_sorted.iter() {
            map.insert(key, value);
        }
        b.iter(|| {
            for (key, _) in keyset_sorted.as_slice() {
                let value = map.get(key);
                if value.is_none() {
                    panic!();
                }
            }
        });
    });
    group.bench_function("FnvHashMap", |b| {
        let mut map = FnvHashMap::default();
        for (key, value) in keyset_sorted.iter() {
            map.insert(key, value);
        }
        b.iter(|| {
            for (key, _) in keyset_sorted.as_slice() {
                let value = map.get(key);
                if value.is_none() {
                    panic!();
                }
            }
        });
    });
    group.bench_function("fst", |b| {
        let map = fst::Map::from_iter(
            keyset_sorted
                .iter()
                .map(|(key, value)| (key, *value as u64)),
        )
        .unwrap();
        b.iter(|| {
            for (key, _) in keyset_sorted.as_slice() {
                let value = map.get(key);
                if value.is_none() {
                    panic!();
                }
            }
        });
    });
    group.bench_function("exact_match_search", |b| {
        let da_bytes = DoubleArrayBuilder::build(keyset_sorted.as_slice()).unwrap();
        let da = DoubleArray::new(da_bytes);
        b.iter(|| {
            for (key, _) in keyset_sorted.as_slice() {
                let value = da.exact_match_search(key);
                if value.is_none() {
                    panic!();
                }
            }
        });
    });
    group.bench_function("common_prefix_search", |b| {
        let da_bytes = DoubleArrayBuilder::build(keyset_sorted.as_slice()).unwrap();
        let da = DoubleArray::new(da_bytes);
        b.iter(|| {
            for (key, _) in keyset_sorted.as_slice() {
                let values = da.common_prefix_search(key);
                let num_matches = values.count();
                if num_matches < 1 {
                    panic!();
                }
            }
        });
    });
    group.finish();
}

fn bench_search_random(c: &mut Criterion) {
    let keyset_sorted = load_ipadic();
    println!(
        "Start a search benchmark by random ordered keys. #keys: {}",
        keyset_sorted.len()
    );

    // randomized keyset
    let mut rng = thread_rng();
    let mut keyset_randomized = keyset_sorted.clone();
    keyset_randomized.as_mut_slice().shuffle(&mut rng);

    let mut group = c.benchmark_group("search/random");
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(5));
    group.sampling_mode(SamplingMode::Flat);

    group.bench_function("BTreeMap", |b| {
        let mut map = BTreeMap::new();
        for (key, value) in keyset_sorted.iter() {
            map.insert(key, value);
        }
        b.iter(|| {
            for (key, _) in keyset_randomized.iter() {
                let value = map.get(key);
                if value.is_none() {
                    panic!();
                }
            }
        });
    });
    group.bench_function("HashMap", |b| {
        let mut map = HashMap::new();
        for (key, value) in keyset_sorted.iter() {
            map.insert(key, value);
        }
        b.iter(|| {
            for (key, _) in keyset_randomized.iter() {
                let value = map.get(key);
                if value.is_none() {
                    panic!();
                }
            }
        });
    });
    group.bench_function("FnvHashMap", |b| {
        let mut map = FnvHashMap::default();
        for (key, value) in keyset_sorted.iter() {
            map.insert(key, value);
        }
        b.iter(|| {
            for (key, _) in keyset_randomized.iter() {
                let value = map.get(key);
                if value.is_none() {
                    panic!();
                }
            }
        });
    });
    group.bench_function("fst", |b| {
        let map = fst::Map::from_iter(
            keyset_sorted
                .iter()
                .map(|(key, value)| (key, *value as u64)),
        )
        .unwrap();
        b.iter(|| {
            for (key, _) in keyset_randomized.iter() {
                let value = map.get(key);
                if value.is_none() {
                    panic!();
                }
            }
        });
    });
    group.bench_function("exact_match_search", |b| {
        let da_bytes = DoubleArrayBuilder::build(keyset_sorted.as_slice()).unwrap();
        let da = DoubleArray::new(da_bytes);
        b.iter(|| {
            for (key, _) in keyset_randomized.iter() {
                let value = da.exact_match_search(key);
                if value.is_none() {
                    panic!();
                }
            }
        });
    });
    group.bench_function("common_prefix_search", |b| {
        let da_bytes = DoubleArrayBuilder::build(keyset_sorted.as_slice()).unwrap();
        let da = DoubleArray::new(da_bytes);
        b.iter(|| {
            for (key, _) in keyset_randomized.as_slice() {
                let values = da.common_prefix_search(key);
                let num_matches = values.count();
                if num_matches < 1 {
                    panic!();
                }
            }
        });
    });
    group.finish();
}

fn load_ipadic() -> Vec<(String, u32)> {
    let file = File::open("data/ipadic-2.7.0.tsv").unwrap();
    let mut keyset: Vec<(String, u32)> = vec![];
    for s in BufReader::new(file).lines() {
        let line = s.ok().unwrap();
        let mut pair = line.split('\t').take(2);
        let key = pair.next().unwrap().to_string();
        let value: u32 = pair.next().unwrap().parse().unwrap();
        keyset.push((key, value));
    }

    keyset
}

criterion_group!(
    benches,
    bench_build,
    bench_search_sorted,
    bench_search_random
);
criterion_main!(benches);
