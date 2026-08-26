//@ run-pass
//@ aux-build: defaults.rs

#![feature(function_param_defaults)]
#![allow(incomplete_features)]

extern crate defaults;

fn main() {
    assert_eq!(defaults::from_other_crate(5), 22);
    assert_eq!(defaults::from_other_crate(5, 9), 14);

    let absent: Option<String> = defaults::generic();
    assert_eq!(absent, None);

    assert_eq!(defaults::const_generic::<12>(), 12);
    assert_eq!(defaults::call_private_callback(), 23);
    assert_eq!(defaults::read_private_static(), 29);
    assert_eq!(defaults::Number(10).increment(), 16);
}
