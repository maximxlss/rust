#![feature(function_param_defaults)]
#![allow(incomplete_features)]

fn add(a: i32, b: i32 = 2) -> i32 {
    a + b
}

fn through_generic<F: Fn(i32, i32) -> i32>(callable: F) {
    let _ = callable(1);
    //~^ ERROR this function takes 2 arguments but 1 argument was supplied
}

fn main() {
    let pointer: fn(i32, i32) -> i32 = add;
    let _ = pointer(1);
    //~^ ERROR this function takes 2 arguments but 1 argument was supplied

    let _: fn(i32) -> i32 = add;
    //~^ ERROR mismatched types

    through_generic(add);
}
