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

func messagesToString(messages []kbls.Fr) string {
	res := ""
	for _, i := range messages {
		res += frToString(&i)
		res += ";"
	}
	return res
}

func generateParams() string {
	srs := C.GoString(C.generate_params_1())
	return srs
}

func commit(srs string, messages []kbls.Fr) string {
	messagesStr := messagesToString(messages)
	srsInput := C.CString(srs)
	defer C.free(unsafe.Pointer(srsInput))
	messagesInput := C.CString(messagesStr)
	defer C.free(unsafe.Pointer(messagesInput))
	output := C.GoString(C.commit_1(srsInput, messagesInput))
	return output
}

func open(srs string, messages []kbls.Fr, pos int) string {
	messagesStr := messagesToString(messages)
	srsInput := C.CString(srs)
	defer C.free(unsafe.Pointer(srsInput))
	messagesInput := C.CString(messagesStr)
	defer C.free(unsafe.Pointer(messagesInput))
	posInput := C.int(pos)
	output := C.GoString(C.open_1(srsInput, messagesInput, posInput))
	return output
}

func verify(srs string, commitment string, message kbls.Fr, pos int, witness string) bool {
	srsInput := C.CString(srs)
	defer C.free(unsafe.Pointer(srsInput))

	commitmentInput := C.CString(commitment)
	defer C.free(unsafe.Pointer(commitmentInput))

	messageStr := frToString(&message)
	messageInput := C.CString(messageStr)
	defer C.free(unsafe.Pointer(messageInput))

	posInput := C.int(pos)

	witnessInput := C.CString(witness)
	defer C.free(unsafe.Pointer(witnessInput))
	output := C.verify_1(srsInput, commitmentInput, messageInput, posInput, witnessInput)
	return output != 0
}

func main() {
	var messages []kbls.Fr
	for i := 0; i < 4; i++ {
		fr := kbls.Fr{}
		if _, err := fr.Rand(rand.Reader); err != nil {
			panic("")
		}
		fr.ToRed()
		messages = append(messages, fr)
	}
	srs := generateParams()
	commitment := commit(srs, messages)
	witness := open(srs, messages, 2)
	if !verify(srs, commitment, messages[2], 2, witness) {
		panic("")
	}
}
