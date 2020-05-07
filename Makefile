build:
	cargo build

install:
	rustup install stable
	rustup default stable
	cargo build --release
	cp ./target/release/cellrs /usr/bin/
	chmod +x /usr/bin/cellrs

ship-linux:
	cargo build --release --target x86_64-unknown-linux-gnu &&\
	cp ./target/x86_64-unknown-linux-gnu/release/cellrs ./cellrs-linux

ship-osx:
	cargo build --release --target x86_64-apple-darwin &&\
	cp ./target/x86_64-apple-darwin/release/cellrs ./cellrs-osx