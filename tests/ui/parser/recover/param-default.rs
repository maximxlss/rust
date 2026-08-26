fn foo(x: i32 = 1) {}
//~^ ERROR default values for function parameters are experimental

type Foo = fn(i32 = 0);
//~^ ERROR default function parameters are not allowed in function pointer types
//~| ERROR default values for function parameters are experimental

fn main() {}
