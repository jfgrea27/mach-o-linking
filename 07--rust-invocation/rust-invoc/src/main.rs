use libloading::Library;
use libloading::Symbol;

fn main() {
    // invocation of c dylib
    unsafe {
        let lib = Library::new("../libfoo.dylib").unwrap();
        let foo_u64_fn: Symbol<unsafe extern "C" fn(u64, u64) -> u64> = lib.get(b"foo\0").unwrap();

        let res = foo_u64_fn(42, 24);
        println!("C lib: Correct result for foo_64: 42 + 24 = {res}");
    }
    // invocation of c dylib wrong type - should fail
    unsafe {
        let lib = Library::new("../libfoo.dylib").unwrap();
        let foo_wrong_fn: Symbol<unsafe extern "C" fn(char, u64) -> u64> =
            lib.get(b"foo\0").unwrap();

        let res = foo_wrong_fn('a', 24); // this does not prevent warning since dylib treats chars as ints.
        println!("C lib: Wrong result for foo_wrong! - 'a' + 24 != {res}");
    }

    // invocation of rust dylib
    unsafe {
        let lib = Library::new("../librust_lib.dylib").unwrap();
        let foo_u64_fn: Symbol<unsafe extern "C" fn(u64, u64) -> u64> =
            lib.get(b"foo_func\0").unwrap();

        let res = foo_u64_fn(42, 24);
        println!("Rust lib: Correct result for foo_u64! - 42 + 24 != {res}");
    }
}
