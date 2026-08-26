# `function_param_defaults`

The tracking issue for this feature is not yet available.

------------------------

The `function_param_defaults` feature allows trailing function parameters to
declare constant default values. Calls to a statically known function item may
omit any trailing suffix for which every parameter has a default:

```rust
#![feature(function_param_defaults)]

fn retry(attempts: u8 = 3, backoff_ms: u64 = 100) {
    // ...
}

retry();
retry(5);
retry(5, 250);
```

Defaults are evaluated as constant expressions in the declaration's scope and
may use the declaration's generic parameters. They cannot refer to runtime
parameters or `self`.

This initial implementation does not support defaults for parameter types that
contain a late-bound lifetime. Explicitly `'static` reference types are
supported.

Parameter defaults do not change a function's type or ABI. A function pointer
and a value called through an `Fn` trait still require the full argument list:

```rust,compile_fail
#![feature(function_param_defaults)]

fn add(a: i32, b: i32 = 1) -> i32 {
    a + b
}

fn main() {
    let pointer: fn(i32, i32) -> i32 = add;
    pointer(2); // error: the full function-pointer arity is required
}
```

This initial experiment supports free functions and inherent methods using the
Rust ABI. Defaults on closures, function pointer declarations, foreign
functions, trait methods, C-variadic functions, and splatted functions are not
supported.
