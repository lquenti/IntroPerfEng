/// Slowest reasonable solution
pub fn matmul1(a: Vec<Vec<f64>>, b: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
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


/// Next improvement: Use stack

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
