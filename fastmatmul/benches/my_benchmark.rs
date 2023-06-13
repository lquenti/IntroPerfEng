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

pub fn matmul3(a: &[[f32; 3]; 3], b: &[[f32; 3]; 3], result: &mut [ [f32; 3]; 3]) {
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
}

fn driver_code3(a: &[[f32; 3]; 3], b: &[[f32; 3]; 3], c: &[[f32; 3]; 3], res_buf: &mut [[f32; 3]; 3]) {
    let mut temp = [[0.0; 3]; 3];
    matmul3(a, b, &mut temp);
    matmul3(&temp, c, res_buf);
}


pub fn matmul4(a: &[f32; 9], b: &[f32; 9], result: &mut [f32; 9]) {
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                result[i * 3 + j] += a[i * 3 + k] * b[k * 3 + j];
            }
        }
    }
}

fn driver_code4(a: &[f32; 9], b: &[f32; 9], c: &[f32; 9], res_buf: &mut [f32; 9]) {
    let mut temp = [0.0; 9];
    matmul4(a, b, &mut temp);
    matmul4(&temp, c, res_buf);
}

fn matmul_benchmark(crit: &mut Criterion) {
    let a1 = vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ];
    let b1 = vec![
        vec![9.0, 8.0, 7.0],
        vec![6.0, 5.0, 4.0],
        vec![3.0, 2.0, 1.0],
    ];
    let c1 = vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
    ];
    let a2 = a1.clone();
    let b2 = b1.clone();
    let c2 = c1.clone();

    let a3 = [
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
        [7.0, 8.0, 9.0],
    ];
    let b3 = [
        [9.0, 8.0, 7.0],
        [6.0, 5.0, 4.0],
        [3.0, 2.0, 1.0],
    ];
    let c3 = [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ];
    let mut result3 = [
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
    ];

    let a4 = [
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0,
    ];
    let b4 = [
        9.0, 8.0, 7.0,
        6.0, 5.0, 4.0,
        3.0, 2.0, 1.0,
    ];
    let c4 = [
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
    ];
    let mut result4 = [0.0; 9];

    let mut group = crit.benchmark_group("Call by");
    group.bench_function("Value", |bench| bench.iter(|| driver_code1(
                black_box(a1.clone()),
                black_box(b1.clone()),
                black_box(c1.clone())
    )));
    group.bench_function("Reference", |bench| bench.iter(|| driver_code2(
                black_box(&a2),
                black_box(&b2),
                black_box(&c2)
    )));
    group.bench_function("Less Heap", |bench| bench.iter(|| driver_code3(
                black_box(&a3),
                black_box(&b3),
                black_box(&c3),
                black_box(&mut result3)
    )));
    group.bench_function("Less Deref", |bench| bench.iter(|| driver_code4(
                black_box(&a4),
                black_box(&b4),
                black_box(&c4),
                black_box(&mut result4)
    )));
}

criterion_group!(benches, matmul_benchmark);
criterion_main!(benches);

