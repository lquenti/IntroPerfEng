pub fn matmul(a: &[[f32; 3]; 3], b: &[[f32; 3]; 3],
              result: &mut [ [f32; 3]; 3]) {
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                result[i][j] += a[i][k] * b[k][j];
            }
}}}
