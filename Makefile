.PHONY: serve serve-release build

serve:
	dx serve --port 8888 --fullstack true

serve-release:
	dx serve --port 8888 --fullstack true --release --profile wasm-release

build:
	dx build --web --fullstack true --release --profile wasm-release
