fn matmul2(a: &[f32], b: &[f32]) -> Vec<f32> {
    let n = (a.len() as f32).sqrt() as usize;
    let mut result = vec![0.0; n * n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                result[i * n + j] += a[i * n + k] * b[k * n + j];
            }
        }
    }
    result
}
