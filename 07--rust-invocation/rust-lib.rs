#[export_name = "foo_func"]
pub extern "C" fn foo(a: u64, b: u64) -> u64 {
    return a + b;
}
