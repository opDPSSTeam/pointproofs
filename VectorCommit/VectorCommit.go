package VectorCommit

/*
#cgo LDFLAGS: -L../target/release -lvc_api
#include <stdlib.h>
#include "./vector_commit.h"
*/
import "C"

import (
	"encoding/base64"
	"unsafe"

	kbls "github.com/kilic/bls12-381"
)

const MaxLength = 64

type VectorCommit struct {
	index int
	srs   string
}

func New(vectorLen int) *VectorCommit {
	if vectorLen < 4 || vectorLen > MaxLength || vectorLen%3 != 1 {
		return nil
	}
	index := (vectorLen - 4) / 3
	indexInput := C.int(index)
	return &VectorCommit{
		index: index,
		srs:   C.GoString(C.generate_params(indexInput)),
	}
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

func (v *VectorCommit) Commit(messages []kbls.Fr) string {
	indexInput := C.int(v.index)
	srsInput := C.CString(v.srs)
	defer C.free(unsafe.Pointer(srsInput))
	messagesStr := messagesToString(messages)
	messagesInput := C.CString(messagesStr)
	defer C.free(unsafe.Pointer(messagesInput))
	output := C.GoString(C.commit(indexInput, srsInput, messagesInput))
	return output
}

func (v *VectorCommit) Open(messages []kbls.Fr, pos int) string {
	indexInput := C.int(v.index)
	messagesStr := messagesToString(messages)
	srsInput := C.CString(v.srs)
	defer C.free(unsafe.Pointer(srsInput))
	messagesInput := C.CString(messagesStr)
	defer C.free(unsafe.Pointer(messagesInput))
	posInput := C.int(pos)
	output := C.GoString(C.open_(indexInput, srsInput, messagesInput, posInput))
	return output
}

func (v *VectorCommit) Verify(commitment string, message kbls.Fr, pos int, witness string) bool {
	indexInput := C.int(v.index)

	srsInput := C.CString(v.srs)
	defer C.free(unsafe.Pointer(srsInput))

	commitmentInput := C.CString(commitment)
	defer C.free(unsafe.Pointer(commitmentInput))

	messageStr := frToString(&message)
	messageInput := C.CString(messageStr)
	defer C.free(unsafe.Pointer(messageInput))

	posInput := C.int(pos)

	witnessInput := C.CString(witness)
	defer C.free(unsafe.Pointer(witnessInput))
	output := C.verify(indexInput, srsInput, commitmentInput, messageInput, posInput, witnessInput)
	return output != 0
}
