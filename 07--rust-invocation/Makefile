all: run

libfoo.dylib:
	clang -o foo.o -c foo.s
	clang -shared -o libfoo.dylib foo.o

run: libfoo.dylib
	cd rust-invoc && cargo run

clean:
	rm -f run *.o libfoo.dylib

@PHONY: clean dis
