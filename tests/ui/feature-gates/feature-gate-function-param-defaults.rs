fn defaulted(value: i32 = 1) -> i32 {
    //~^ ERROR default values for function parameters are experimental
    value
}

fn main() {
    let _ = defaulted();
    //~^ ERROR omitting function arguments with default values is experimental
}
