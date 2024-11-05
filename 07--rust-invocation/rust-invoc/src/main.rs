use libloading::Library;
use libloading::Symbol;

fn main() {
    unsafe {
        let lib = Library::new("../libfoo.dylib").unwrap();
        let foo_u64_fn: Symbol<unsafe extern "C" fn(u64, u64) -> u64> = lib.get(b"foo\0").unwrap();

        let res = foo_u64_fn(42, 24);

        if res != 42 + 24 {
            println!("Wrong result for foo_u64! - 42 + 24 != {res}");
        } else {
            println!("Correct result for foo_64: 42 + 24 = {res}");
        }
    }

    unsafe {
        let lib = Library::new("../libfoo.dylib").unwrap();
        let foo_f64_fn: Symbol<unsafe extern "C" fn(f64, f64) -> f64> = lib.get(b"foo\0").unwrap();

        let res = foo_f64_fn(42.2, 24.2);

        if res != 42.2 + 24.2 {
            println!("Wrong result for foo_f64! - 42.2 + 24.2 != {res}");
        } else {
            println!("Correct result for foo_f64: 42.2 + 24.2 = {res}");
        }
    }
}
