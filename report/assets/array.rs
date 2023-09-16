fn matmul(a: &[[f32; 3]; 3], b: &[[f32; 3]; 3], result: &mut [ [f32; 3]; 3]) {
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
}

fn driver_code(a: &[[f32; 3]; 3], b: &[[f32; 3]; 3], c: &[[f32; 3]; 3],
               res_buf: &mut [[f32; 3]; 3]) {
    let mut temp = [[0.0; 3]; 3];
    matmul3(a, b, &mut temp);
    matmul3(&temp, c, res_buf);
}
