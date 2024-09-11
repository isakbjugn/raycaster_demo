build:
	cargo build --release

bundle:
	cp target/wasm32-unknown-unknown/release/raycaster_demo.wasm .
	wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code raycaster_demo.wasm -o raycaster_demo.wasm

html:
	rm -f labyrint.html
	w4 bundle raycaster_demo.wasm --title "Labyrint" --html labyrint.html

size:
	du -bh ./target/wasm32-unknown-unknown/release/raycaster_demo.wasm

run: build bundle
	w4 run-native raycaster_demo.wasm

run-web: build-web bundle
	w4 run --no-qr raycaster_demo.wasm
	# w4 run --no-qr --no-open raycaster_demo.wasm

dev:
	cargo watch -s "make run"