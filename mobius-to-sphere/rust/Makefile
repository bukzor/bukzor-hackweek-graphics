all: devenv host

host:
	direnv exec . cargo watch --exec=run
wasm:
	direnv exec . cargo watch --exec='run --target wasm32-unknown-unknown'
devenv:
	brew bundle
	cargo install wasm-server-runner --root ./node_modules/
	direnv allow
