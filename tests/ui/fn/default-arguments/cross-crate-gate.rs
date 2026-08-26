//@ aux-build: defaults.rs

extern crate defaults;

fn main() {
    let _ = defaults::from_other_crate(5);
    //~^ ERROR omitting function arguments with default values is experimental
}
