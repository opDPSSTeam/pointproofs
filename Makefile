build:
	cd rust-demo && cargo build --release
	go build -o go-rust -ldflags="-r ./target/release" main.go

clean:
	rm go-rust 
