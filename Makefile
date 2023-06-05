.PHONY: build clean

build:
	cd rust-demo && cargo build --release
	go build -o go-rust -ldflags="-r ./target/release" main.go

run: build
	./go-rust

clean:
	rm -rf target
	rm go-rust 
