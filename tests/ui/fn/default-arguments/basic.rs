//@ run-pass
//@ edition: 2024

#![feature(function_param_defaults)]
#![allow(incomplete_features)]

fn combine(a: i32, b: i32 = 20, c: i32 = 3) -> i32 {
    a * 100 + b * 10 + c
}

fn empty_len(values: Vec<u8> = Vec::new()) -> usize {
    values.len()
}

fn generic<const N: usize>(value: usize = N) -> usize {
    value
}

fn maybe<T>(value: Option<T> = None) -> Option<T> {
    value
}

fn static_ref(value: Option<&'static str> = None) -> Option<&'static str> {
    value
}

fn early_ref<'a>(value: Option<&'a str> = None) -> Option<&'a str>
where
    'a: 'a,
{
    value
}

fn string_len(value: &str) -> usize {
    value.len()
}

fn higher_ranked(callback: for<'a> fn(&'a str) -> usize = string_len) -> usize {
    callback("default")
}

trait HasValue {
    const VALUE: usize;
}

struct Value;

impl HasValue for Value {
    const VALUE: usize = 21;
}

fn associated_const<T: HasValue>(value: usize = T::VALUE) -> usize {
    value
}

macro_rules! default_number {
    () => {
        19
    };
}

fn macro_default(value: i32 = default_number!()) -> i32 {
    value
}

const fn const_add(a: i32, b: i32 = 5) -> i32 {
    a + b
}

const CONST_RESULT: i32 = const_add(7);

struct Counter(i32);

impl Counter {
    const STEP: i32 = 4;

    fn add(&self, amount: i32 = Self::STEP) -> i32 {
        self.0 + amount
    }

    fn associated(value: i32 = 9) -> i32 {
        value
    }
}

async fn async_value(value: i32 = 11) -> i32 {
    value
}

extern "Rust" fn explicit_rust_abi(value: i32 = 13) -> i32 {
    value
}

unsafe fn unsafe_value(value: i32 = 15) -> i32 {
    value
}

fn main() {
    assert_eq!(combine(1), 303);
    assert_eq!(combine(1, 4), 143);
    assert_eq!(combine(1, 4, 8), 148);

    let item = combine;
    assert_eq!(item(2), 403);

    let pointer: fn(i32, i32, i32) -> i32 = combine;
    assert_eq!(pointer(2, 5, 7), 257);

    assert_eq!(empty_len(), 0);
    assert_eq!(generic::<7>(), 7);
    assert_eq!(generic::<7>(12), 12);

    let absent: Option<String> = maybe();
    assert_eq!(absent, None);
    assert_eq!(maybe(Some(String::from("hello"))).as_deref(), Some("hello"));
    assert_eq!(static_ref(), None);
    let early: Option<&'static str> = early_ref::<'static>();
    assert_eq!(early, None);
    assert_eq!(higher_ranked(), 7);
    assert_eq!(associated_const::<Value>(), 21);
    assert_eq!(macro_default(), 19);

    assert_eq!(CONST_RESULT, 12);
    assert_eq!(Counter(10).add(), 14);
    assert_eq!(Counter(10).add(8), 18);
    assert_eq!(Counter::add(&Counter(10)), 14);
    assert_eq!(Counter::associated(), 9);
    assert_eq!(explicit_rust_abi(), 13);
    assert_eq!(unsafe { unsafe_value() }, 15);

    drop(async_value());
}
