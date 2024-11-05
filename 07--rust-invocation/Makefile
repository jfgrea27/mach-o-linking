all: run

libfoo.dylib:
	clang -o foo.o -c foo.s
	clang -shared -o libfoo.dylib foo.o

rustfoo.dylib:
	rustc --crate-type=dylib rust-lib.rs

run: libfoo.dylib rustfoo.dylib
	cd rust-invoc && cargo run

clean:
	rm -f run *.o *.dylib

@PHONY: clean dis
