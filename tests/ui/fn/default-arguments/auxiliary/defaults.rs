#![feature(function_param_defaults)]
#![allow(incomplete_features)]

const PRIVATE_DEFAULT: i32 = 17;

fn private_callback() -> i32 {
    23
}

static PRIVATE_STATIC: i32 = 29;

pub fn from_other_crate(a: i32, b: i32 = PRIVATE_DEFAULT) -> i32 {
    a + b
}

pub fn generic<T>(value: Option<T> = None) -> Option<T> {
    value
}

pub fn const_generic<const N: usize>(value: usize = N) -> usize {
    value
}

pub fn call_private_callback(callback: fn() -> i32 = private_callback) -> i32 {
    callback()
}

pub fn read_private_static(value: &'static i32 = &PRIVATE_STATIC) -> i32 {
    *value
}

pub struct Number(pub i32);

impl Number {
    const INCREMENT: i32 = 6;

    pub fn increment(&self, amount: i32 = Self::INCREMENT) -> i32 {
        self.0 + amount
    }
}
