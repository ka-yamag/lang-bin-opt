package main

import (
	"crypto/subtle"
	"fmt"
)

func main() {
	a := []byte("abcdefghijklkm012345")
	b := []byte("012345:;++)))(*(&*^%kadjfklajfklaikadjfkladjfkladjsj")

	if subtle.ConstantTimeCompare(a, b) == 1 {
		fmt.Println("match!")
	} else {
		fmt.Println("unmatch!")
	}
}
