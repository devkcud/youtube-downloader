all: compile

compile:
	cargo build --release

install: compile
	sudo cp ./target/release/ytdp /usr/local/bin/ytdp
