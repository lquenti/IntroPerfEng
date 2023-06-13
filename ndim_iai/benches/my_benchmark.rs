use iai::black_box;

/// Slowest reasonable solution
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

fn matmul2(a: &[Vec<f32>], b: &[Vec<f32>]) -> Vec<Vec<f32>> {
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


fn iai_normal() {
    let n = 1024;
    let a = vec![2.0; n*n];
    let b = vec![3.0; n*n];
    let mut res = vec![0.0; n*n];
    matmul3(
                black_box(&a),
                black_box(&b),
                black_box(&mut res),
                n
    );
}

fn iai_transpose() {
    let n = 1024;
    let a = vec![2.0; n*n];
    let b = vec![3.0; n*n];
    let mut res = vec![0.0; n*n];
    matmul4(
                black_box(&a),
                black_box(&b),
                black_box(&mut res),
                n
    );
}

iai::main!{iai_normal, iai_transpose}
