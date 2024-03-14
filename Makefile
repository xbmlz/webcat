run: build-css
	@RUST_LOG=info cargo run

run-release: build-css
	@RUST_LOG=info cargo run --release

watch:
	@watchexec --restart --exts rs,js,css,html --ignore public -- make run

build-css:
	@npm run build