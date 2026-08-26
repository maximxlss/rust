#![feature(function_param_defaults)]

fn defaults(required:i32,arithmetic:i32=1+2) {}

fn comments(value:i32/* before */=/* after */3) {}
