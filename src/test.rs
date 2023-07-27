use super::*;
#[test]
fn create() {
    set_backend(["optix"]).unwrap();

    type Float32 = Var<f32>;

    let x = Float32::from(1.);

    let z = x + 1.;

    z.0.schedule();

    eval();

    let z: Vec<_> = z.into();
    dbg!(z);
}
