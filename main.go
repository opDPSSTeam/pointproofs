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
)

func SubModFr(dst *kbls.Fr, a, b *kbls.Fr) {
	(*kbls.Fr)(dst).Sub((*kbls.Fr)(a), (*kbls.Fr)(b))
}

func AddModFr(dst *kbls.Fr, a, b *kbls.Fr) {
	(*kbls.Fr)(dst).Add((*kbls.Fr)(a), (*kbls.Fr)(b))
}

func main() {
	fr1 := kbls.Fr{}
	if _, err := fr1.Rand(rand.Reader); err != nil {
		panic("")
	}
	fr1.ToRed()
	bytes := fr1.ToBytes()
	for i, j := 0, len(bytes)-1; i < j; i, j = i+1, j-1 {
		bytes[i], bytes[j] = bytes[j], bytes[i]
	}

	s1 := base64.StdEncoding.EncodeToString(bytes)
	fr2 := kbls.Fr{}
	if _, err := fr2.Rand(rand.Reader); err != nil {
		panic("")
	}
	fr2.ToRed()
	bytes = fr2.ToBytes()
	for i, j := 0, len(bytes)-1; i < j; i, j = i+1, j-1 {
		bytes[i], bytes[j] = bytes[j], bytes[i]
	}

	s2 := base64.StdEncoding.EncodeToString(bytes)
	input1 := C.CString(s1)
	//defer C.free(unsafe.Pointer(input))
	//
	input2 := C.CString(s2)
	output := C.GoString(C.fr_plus(input1, input2))
	bytes, _ = base64.StdEncoding.DecodeString(output)
	for i, j := 0, len(bytes)-1; i < j; i, j = i+1, j-1 {
		bytes[i], bytes[j] = bytes[j], bytes[i]
	}
	fr3 := kbls.Fr{}
	fr3.FromBytes(bytes)
	AddModFr(&fr1, &fr1, &fr2)
	if !fr3.Equal(&fr1) {
		panic("")
	}
}
