#![feature(function_param_defaults)]
#![allow(incomplete_features)]

fn runtime() -> i32 {
    1
}

fn non_const(value: i32 = runtime()) {}
//~^ ERROR cannot call non-const function `runtime` in constants

fn wrong_type(value: i32 = "not an integer") {}
//~^ ERROR mismatched types

fn references_parameter(first: i32, second: i32 = first) {}
//~^ ERROR cannot find value `first` in this scope

fn elided_lifetime(value: &str = "default") {}
//~^ ERROR defaults for parameters with late-bound lifetimes are not supported

fn explicit_lifetime<'a>(value: Option<&'a str> = None) {}
//~^ ERROR defaults for parameters with late-bound lifetimes are not supported

fn still_required(required: i32, optional: i32 = 2) {}

fn main() {
    still_required();
    //~^ ERROR this function takes 1 argument but 0 arguments were supplied

    still_required("wrong");
    //~^ ERROR mismatched types
}
