#![feature(function_param_defaults)]
#![allow(incomplete_features)]

fn required_after_default(first: i32 = 1, second: i32) {}
//~^ ERROR parameters without a default cannot follow parameters with defaults

type FunctionPointer = fn(value: i32 = 1);
//~^ ERROR default function parameters are not allowed in function pointer types

extern "Rust" {
    fn foreign(value: i32 = 1);
    //~^ ERROR default function parameters are not allowed in foreign function declarations
}

extern "C" fn non_rust_abi(value: i32 = 1) {}
//~^ ERROR default function parameters are not allowed in functions with a non-Rust ABI

trait TraitMethod {
    fn method(value: i32 = 1);
    //~^ ERROR default function parameters are not allowed in trait methods
}

struct Value;

impl Value {
    fn receiver(self = Value) {}
    //~^ ERROR the `self` parameter cannot have a default
}

impl TraitMethod for Value {
    fn method(value: i32 = 1) {}
    //~^ ERROR default function parameters are not allowed in trait implementation methods
}

fn main() {}
