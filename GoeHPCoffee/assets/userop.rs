let mut h = 0;
comm.all_reduce_into(
    &(rank + 1),
    &mut h,
    &UserOperation::commutative(|x, y| {
        let x: &[Rank] = x.downcast().unwrap();
        let y: &mut [Rank] = y.downcast().unwrap();
        for (&x_i, y_i) in x.iter().zip(y) {
            *y_i += x_i;
        }
    }),
);
