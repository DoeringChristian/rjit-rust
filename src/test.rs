use crate::full;

use super::*;
#[test]
fn create() {
    pretty_env_logger::try_init().ok();
    set_backend(["optix"]).unwrap();

    let x = array(&[1u16, 2]);
    let y = array(&[1u16, 3]);

    let z = x.max(y);

    z.schedule();

    eval();

    let z: Vec<_> = z.into();
    dbg!(z);
}
#[test]
fn full_1() {
    pretty_env_logger::try_init().ok();
    set_backend(["optix"]).unwrap();

    let x = full(1., 100);

    x.schedule();

    eval();

    let x: Vec<_> = x.into();
    assert_eq!(x, vec![1.; 100]);
}
