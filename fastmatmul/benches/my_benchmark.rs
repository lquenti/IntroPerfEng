use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, black_box};

/// Slowest reasonable solution
pub fn matmul1(a: Vec<Vec<f32>>, b: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut result = vec![vec![0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}

fn driver_code1(a: Vec<Vec<f32>>, b: Vec<Vec<f32>>, c: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    matmul1(matmul1(a, b), c)
}

/// Next improvement: Use references
pub fn matmul2(a: &Vec<Vec<f32>>, b: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut result = vec![vec![0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}

fn driver_code2(a: &Vec<Vec<f32>>, b: &Vec<Vec<f32>>, c: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    matmul2(&matmul2(a, b), c)
}

fn matmul1_benchmark(crit: &mut Criterion) {
    let a = vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ];
    let b = vec![
        vec![9.0, 8.0, 7.0],
        vec![6.0, 5.0, 4.0],
        vec![3.0, 2.0, 1.0],
    ];
    let c = vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
    ];
    crit.bench_function("matmul1", |bench| bench.iter(|| driver_code1(
                black_box(a.clone()),
                black_box(b.clone()),
                black_box(c.clone())
    )));
}
fn matmul2_benchmark(crit: &mut Criterion) {
    let a = vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ];
    let b = vec![
        vec![9.0, 8.0, 7.0],
        vec![6.0, 5.0, 4.0],
        vec![3.0, 2.0, 1.0],
    ];
    let c = vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
    ];
    crit.bench_function("matmul2", |bench| bench.iter(|| driver_code2(
                black_box(&a),
                black_box(&b),
                black_box(&c)
    )));
}

criterion_group!(benches, matmul1_benchmark, matmul2_benchmark);
criterion_main!(benches);

