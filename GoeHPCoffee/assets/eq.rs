#[derive(Equivalence)]
struct MyProgramOpts {
    name: [u8; 100],
    num_cycles: u32,
    material_properties: [f64; 20],
}
