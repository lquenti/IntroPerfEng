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

/// Next improvement: Use stack
fn matmul3(a: &[[f32; 3]; 3], b: &[[f32; 3]; 3], result: &mut [ [f32; 3]; 3]) {
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
}

/// Next improvement: Use single array
fn matmul4(a: &[f32; 9], b: &[f32; 9], result: &mut [f32; 9]) {
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                result[i * 3 + j] += a[i * 3 + k] * b[k * 3 + j];
            }
        }
    }
}


const A: [[f32; 3]; 3] = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
const B: [[f32; 3]; 3] = [[9.0, 8.0, 7.0], [6.0, 5.0, 4.0], [3.0, 2.0, 1.0]];
const C: [[f32; 3]; 3] = [[4.0, 5.0, 6.0], [7.0, 8.0, 9.0], [1.0, 2.0, 3.0]];

fn driver() {
    let mut res1: [[f32; 3]; 3] = [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];
    let mut res2: [[f32; 3]; 3] = [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];
    matmul3(&A, &B, &mut res1);
    matmul3(&res1, &C, &mut res2);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
