fn matmul3(a: &[f32], b: &[f32], result: &mut [f32], n: usize) {
    result.iter_mut().enumerate().for_each(|(idx, res)| {
        let i = idx / n;
        let j = idx % n;
        *res = (0..n).map(|k| a[i * n + k] * b[k * n + j]).sum();
    });
}
