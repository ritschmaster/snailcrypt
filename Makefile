PROJECT=libsnailcrypt

all:
	cargo auditable build --release

debug:
	cargo build

check:
	cargo test

cbindgen:
	~/.cargo/bin/cbindgen --config cbindgen.toml --crate $(PROJECT) --output target/snailcrypt.h

audit:
	cargo audit bin target/release/snailcrypt

doc:
	cargo doc

publish:
	cargo publish

.PHONY = all \
	debug \
	cbindge \
	audit
