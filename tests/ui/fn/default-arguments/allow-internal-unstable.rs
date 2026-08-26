//@ check-pass
//@ aux-build: defaults.rs

#![feature(allow_internal_unstable)]

extern crate defaults;

#[allow_internal_unstable(function_param_defaults)]
macro_rules! call_with_default {
    () => {
        defaults::from_other_crate(5)
    };
}

fn main() {
    assert_eq!(call_with_default!(), 22);
}
