PROJECT=snailcrypt

all:
	cargo auditable build --release

debug:
	cargo build

check:
	cargo test

check-ctest: debug cbindgen
	mkdir -p target/debug/ctest

	cc -Itarget -o target/debug/ctest/ctest ctest/main.c -Ltarget/debug -lsnailcrypt

	./target/debug/ctest/ctest

cbindgen:
	~/.cargo/bin/cbindgen --config cbindgen.toml --crate $(PROJECT) --output target/snailcrypt.h

audit:
	cargo audit bin target/release/snailcrypt

doc:
	cargo doc

publish:
	cargo publish

.PHONY = all \
	doc \
	debug \
	check \
	cbindgen \
	doc \
	publish
