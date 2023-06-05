package main

/*
#cgo LDFLAGS: -L./target/release -lrust_demo
#include <stdlib.h>
#include "./lib/rustdemo.h"
*/
import "C"

import "fmt"
import "unsafe"

func main() {
	s := "Hello+"
	input := C.CString(s)
	defer C.free(unsafe.Pointer(input))

	o := C.rust_demo(input, 2)
	output := C.GoString(o)
	fmt.Println(output)
}
