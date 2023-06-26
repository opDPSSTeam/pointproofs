package main

import (
	"crypto/rand"
	"fmt"
	kbls "github.com/kilic/bls12-381"
	"go-demo/VectorCommit"
)


func main() {
	var messages []kbls.Fr
	for i := 0; i < 7; i++ {
		fr := kbls.Fr{}
		if _, err := fr.Rand(rand.Reader); err != nil {
			panic("")
		}
		fr.ToRed()
		messages = append(messages, fr)
	}
	vc := VectorCommit.New(7)
	commitment := vc.Commit(messages)
	witness := vc.Open(messages, 2)
	if !vc.Verify(commitment, messages[2], 2, witness) {
		panic("")
	}
	fmt.Println("Success")
}
