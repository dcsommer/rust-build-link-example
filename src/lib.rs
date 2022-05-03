extern "C" {
    fn mystatic_foo() -> i32;
}

#[no_mangle]
pub extern "C" fn callable_from_c(x: i32) -> bool {
    (unsafe { mystatic_foo() }) % x == 0
}
