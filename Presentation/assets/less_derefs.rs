pub fn matmul(a: &[f32; 9], b: &[f32; 9], result: &mut [f32; 9]) {
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                result[i * 3 + j] += a[i * 3 + k] * b[k * 3 + j];
            }
}}}
