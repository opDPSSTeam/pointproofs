package main

/*
#cgo LDFLAGS: -L./target/release -lvc_api
#include <stdlib.h>
#include "./lib/rustdemo.h"
*/
import "C"
import (
	"crypto/rand"
	"encoding/base64"
	kbls "github.com/kilic/bls12-381"
	"unsafe"
)

func SubModFr(dst *kbls.Fr, a, b *kbls.Fr) {
	(*kbls.Fr)(dst).Sub((*kbls.Fr)(a), (*kbls.Fr)(b))
}

func AddModFr(dst *kbls.Fr, a, b *kbls.Fr) {
	(*kbls.Fr)(dst).Add((*kbls.Fr)(a), (*kbls.Fr)(b))
}

func frToString(fr *kbls.Fr) string {
	bytes := fr.ToBytes()
	for i, j := 0, len(bytes)-1; i < j; i, j = i+1, j-1 {
		bytes[i], bytes[j] = bytes[j], bytes[i]
	}
	return base64.StdEncoding.EncodeToString(bytes)
}

func stringToFr(s string) kbls.Fr {
	bytes, _ := base64.StdEncoding.DecodeString(s)
	for i, j := 0, len(bytes)-1; i < j; i, j = i+1, j-1 {
		bytes[i], bytes[j] = bytes[j], bytes[i]
	}
	fr := kbls.Fr{}
	fr.FromBytes(bytes)
	return fr
}

func main() {
	fr1 := kbls.Fr{}
	if _, err := fr1.Rand(rand.Reader); err != nil {
		panic("")
	}
	fr1.ToRed()
	s1 := frToString(&fr1)

	fr2 := kbls.Fr{}
	if _, err := fr2.Rand(rand.Reader); err != nil {
		panic("")
	}
	fr2.ToRed()
	s2 := frToString(&fr2)

	input1 := C.CString(s1)
	defer C.free(unsafe.Pointer(input1))
	input2 := C.CString(s2)
	defer C.free(unsafe.Pointer(input2))

	output := C.GoString(C.fr_plus(input1, input2))
	fr3 := stringToFr(output)

	AddModFr(&fr1, &fr1, &fr2)
	if !fr3.Equal(&fr1) {
		panic("")
	}
}
