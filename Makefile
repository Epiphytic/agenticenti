WASM_TARGET := wasm32-wasip1
WASM_OUT := target/$(WASM_TARGET)/release/agenticenti.wasm
NPM_WASM := npm/wasm/agenticenti.wasm

.PHONY: build-native build-wasm test npm-pack npm-publish clean

build-native:
	cargo build --release

build-wasm:
	@rustup target list --installed | grep -q $(WASM_TARGET) || \
		{ echo "Adding $(WASM_TARGET) target..."; rustup target add $(WASM_TARGET); }
	cargo build --target $(WASM_TARGET) --release
	cp $(WASM_OUT) $(NPM_WASM)

test:
	cargo test

npm-pack: build-wasm
	cd npm && npm pack

npm-publish: build-wasm
	cd npm && npm publish

clean:
	cargo clean
	rm -f $(NPM_WASM)
