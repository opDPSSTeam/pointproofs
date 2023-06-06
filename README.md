# Golang 调用 Rust Pointproof

编译：`make build`，产生可执行文件 `go-rust`

目前只能对长度为 4 或 7 的向量进行承诺。对更多向量的承诺需要修改 `VectorCommit/VectorCommit.go` 中的 `MaxLength`。同时修改 `vc_api/src/lib.rs` 下的四个 const 函数指针数组，增加更多的长度。

要将 `VectorCommit` 这个 package 用于其他部分，可能需要修改 `-L../target/release`，让 `-L` 后的路径指向正确的 Rust 编译出的 target。
