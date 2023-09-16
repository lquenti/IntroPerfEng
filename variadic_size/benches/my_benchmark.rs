use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, black_box};

fn matmul1(a: Vec<Vec<f32>>, b: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let n = a.len();
    let mut result = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}

fn matmul3(a: &[f32], b: &[f32], result: &mut [f32], n: usize) {
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                result[i * n + j] += a[i * n + k] * b[k * n + j];
            }
        }
    }
}

fn matmul4(a: &[f32], _b: &[f32], c: &mut [f32], n: usize) {
    let mut b = vec![0.0; n * n];

    for i in 0..n {
        for j in 0..n {
            b[i * n + j] = _b[j * n + i];
        }
    }

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i * n + j] += a[i * n + k] * b[j * n + k];
            }
        }
    }
}

fn matmul_benchmark(crit: &mut Criterion) {
    let n = 1024;
    let a = vec![2.0; n*n];
    let b = vec![3.0; n*n];
    let mut res = vec![0.0; n*n];
    let a2 = vec![2.0; n*n];
    let b2 = vec![3.0; n*n];
    let mut res2 = vec![0.0; n*n];


    let avec: Vec<Vec<f32>> = vec![vec![2.0; n]; n];
    let bvec: Vec<Vec<f32>> = vec![vec![3.0; n]; n];


    let mut group = crit.benchmark_group("Dynamic Size");
    group.bench_function("Unoptimized", |bench| bench.iter(|| matmul1(
                black_box(avec.clone()),
                black_box(bvec.clone())
    )));
    group.bench_function("Optimized", |bench| bench.iter(|| matmul3(
                black_box(&a),
                black_box(&b),
                black_box(&mut res),
                n
    )));
    group.bench_function("Optimized + Transposed", |bench| bench.iter(|| matmul4(
                black_box(&a2),
                black_box(&b2),
                black_box(&mut res2),
                n
    )));
}

criterion_group!(benches, matmul_benchmark);
criterion_main!(benches);

