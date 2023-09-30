
parse_basic: parse_basic.c libdatetimeparse.so
	cc -g -o parse_basic parse_basic.c -L. -ldatetimeparse -I ../include


libdatetimeparse.so: ../include/datetimeparse.h ../src/*.rs
	cargo build --manifest-path ../Cargo.toml
	-cp ../../target/debug/libdatetimeparse.dylib libdatetimeparse.so
	-cp ../../target/debug/libdatetimeparse.so libdatetimeparse.so
