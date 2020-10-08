use criterion::measurement::WallTime;
use criterion::{criterion_group, criterion_main, BenchmarkGroup, Criterion, SamplingMode};
use fnv::FnvHashMap;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;
use yada::builder::DoubleArrayBuilder;
use yada::DoubleArray;

const BUILD_SAMPLE_SIZE: usize = 10;
const BUILD_WARM_UP_TIME: Duration = Duration::from_secs(10);

const SEARCH_SAMPLE_SIZE: usize = 10;
const SEARCH_MEASURE_TIME: Duration = Duration::from_secs(1);

fn bench_build_ipadic(c: &mut Criterion) {
    let keyset = load_ipadic();

    let mut group = c.benchmark_group("build/ipadic");
    group.sample_size(BUILD_SAMPLE_SIZE);
    group.warm_up_time(BUILD_WARM_UP_TIME);
    group.measurement_time(Duration::from_secs(10));
    group.sampling_mode(SamplingMode::Flat);

    group.bench_function("yada", |b| {
        b.iter(|| DoubleArrayBuilder::build(keyset.as_slice()));
    });

    group.finish();
}

fn bench_build_unidic(c: &mut Criterion) {
    let keyset = load_unidic();

    let mut group = c.benchmark_group("build/unidic");
    group.sample_size(BUILD_SAMPLE_SIZE);
    group.warm_up_time(BUILD_WARM_UP_TIME);
    group.measurement_time(Duration::from_secs(15));
    group.sampling_mode(SamplingMode::Flat);

    group.bench_function("yada", |b| {
        b.iter(|| DoubleArrayBuilder::build(keyset.as_slice()));
    });

    group.finish();
}

fn bench_search_sorted_ipadic(c: &mut Criterion) {
    let keyset_sorted = load_ipadic();
    let mut group = c.benchmark_group("search/sorted/ipadic");
    add_search_bench_functions(&mut group, &keyset_sorted, &keyset_sorted);
    group.finish();
}

fn bench_search_sorted_unidic(c: &mut Criterion) {
    let keyset_sorted = load_unidic();
    let mut group = c.benchmark_group("search/sorted/unidic");
    add_search_bench_functions(&mut group, &keyset_sorted, &keyset_sorted);
    group.finish();
}

fn bench_search_random_ipadic(c: &mut Criterion) {
    let keyset_sorted = load_ipadic();

    // randomized keyset
    let mut rng = thread_rng();
    let mut keyset_randomized = keyset_sorted.clone();
    keyset_randomized.as_mut_slice().shuffle(&mut rng);

    let mut group = c.benchmark_group("search/random/ipadic");
    add_search_bench_functions(&mut group, &keyset_sorted, &keyset_randomized);
    group.finish();
}

fn bench_search_random_unidic(c: &mut Criterion) {
    let keyset_sorted = load_unidic();

    // randomized keyset
    let mut rng = thread_rng();
    let mut keyset_randomized = keyset_sorted.clone();
    keyset_randomized.as_mut_slice().shuffle(&mut rng);

    let mut group = c.benchmark_group("search/random/unidic");
    add_search_bench_functions(&mut group, &keyset_sorted, &keyset_randomized);
    group.finish();
}

fn add_search_bench_functions(
    group: &mut BenchmarkGroup<WallTime>,
    keyset_build: &Vec<(String, u32)>,
    keyset_search: &Vec<(String, u32)>,
) {
    group.sample_size(SEARCH_SAMPLE_SIZE);
    group.measurement_time(SEARCH_MEASURE_TIME);
    group.sampling_mode(SamplingMode::Flat);

    group.bench_function("BTreeMap", |b| {
        let mut map = BTreeMap::new();
        for (key, value) in keyset_build.iter() {
            map.insert(key, value);
        }
        b.iter(|| {
            for (key, _) in keyset_search.iter() {
                let value = map.get(key);
                if value.is_none() {
                    panic!();
                }
            }
        });
    });
    group.bench_function("HashMap", |b| {
        let mut map = HashMap::new();
        for (key, value) in keyset_build.iter() {
            map.insert(key, value);
        }
        b.iter(|| {
            for (key, _) in keyset_search.iter() {
                let value = map.get(key);
                if value.is_none() {
                    panic!();
                }
            }
        });
    });
    group.bench_function("FnvHashMap", |b| {
        let mut map = FnvHashMap::default();
        for (key, value) in keyset_build.iter() {
            map.insert(key, value);
        }
        b.iter(|| {
            for (key, _) in keyset_search.iter() {
                let value = map.get(key);
                if value.is_none() {
                    panic!();
                }
            }
        });
    });
    group.bench_function("fst", |b| {
        let map = fst::Map::from_iter(keyset_build.iter().map(|(key, value)| (key, *value as u64)))
            .unwrap();
        b.iter(|| {
            for (key, _) in keyset_search.iter() {
                let value = map.get(key);
                if value.is_none() {
                    panic!();
                }
            }
        });
    });
    group.bench_function("exact_match_search", |b| {
        let da_bytes = DoubleArrayBuilder::build(keyset_build.as_slice()).unwrap();
        let da = DoubleArray::new(da_bytes);
        b.iter(|| {
            for (key, _) in keyset_search.iter() {
                let value = da.exact_match_search(key);
                if value.is_none() {
                    panic!();
                }
            }
        });
    });
    group.bench_function("common_prefix_search", |b| {
        let da_bytes = DoubleArrayBuilder::build(keyset_build.as_slice()).unwrap();
        let da = DoubleArray::new(da_bytes);
        b.iter(|| {
            for (key, _) in keyset_search.as_slice() {
                let values = da.common_prefix_search(key);
                let num_matches = values.count();
                if num_matches < 1 {
                    panic!();
                }
            }
        });
    });
}

fn load_ipadic() -> Vec<(String, u32)> {
    load_dic("data/ipadic-2.7.0.tsv")
}

fn load_unidic() -> Vec<(String, u32)> {
    load_dic("data/unidic-2.1.2.tsv")
}

fn load_dic(path: &str) -> Vec<(String, u32)> {
    let file = File::open(path).unwrap();
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
    bench_build_ipadic,
    bench_build_unidic,
    bench_search_sorted_ipadic,
    bench_search_sorted_unidic,
    bench_search_random_ipadic,
    bench_search_random_unidic,
);
criterion_main!(benches);
