fn matmul(a: &Vec<Vec<f32>>, b: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    /* Only the signature changes... */
}
fn driver_code(a: Vec<Vec<f32>>, b: Vec<Vec<f32>>, c: Vec<Vec<f32>>)
    -> Vec<Vec<f32>> {
    matmul(matmul(a, b), c) // D := A * B * C
}
