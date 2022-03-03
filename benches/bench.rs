#[path="../src/sample.rs"]
mod sample;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use sample::sample::{Point, generate_data, bf_knn};
use vptree::vptree::VPTree;

const LARGE_DATASET_KNN: usize = 10;

fn benchmark(c: &mut Criterion)
{
    let mut knn = c.benchmark_group("knn search");
    // 16 to 8192 nodes
    for i in (4..14).map(|a| 2i32.pow(a))
    {
        let data = generate_data(i);

        // O(n^2) as reference
        knn.bench_function(BenchmarkId::new("brute force", i), |b| b.iter(|| {
            data.iter().for_each(|p| { bf_knn(&data, p, black_box(LARGE_DATASET_KNN)); });
        }));

        // let cp = &mut data.clone();

        // O(2n log(n)) no copy?
        knn.bench_function(BenchmarkId::new("vptree", i), |b| b.iter(|| {
            let tree = VPTree::new(&data, &Point::distance);
            data.iter().for_each(|p| { tree.search(p, black_box(LARGE_DATASET_KNN)); });
        }));
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
