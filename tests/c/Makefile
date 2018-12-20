libstretch.h:
	cd ../../ && cbindgen > ./tests/c/libstretch.h

libstretch.a:
	cargo build\
		-Z unstable-options\
		--lib --out-dir .

test_bindings: libstretch.h libstretch.a
	gcc libstretch.a test_bindings.c -o test_bindings
