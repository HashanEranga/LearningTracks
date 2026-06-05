package main

import (
	"bytes"
	"fmt"
	"os"
	"strings"
	"unicode/utf8"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Fprintln(os.Stderr, "Usage: go-wc <file>")
		os.Exit(1)
	}
	path := os.Args[1]
	data, err := os.ReadFile(path)
	if err != nil {
		fmt.Fprintln(os.Stderr, "error reading", path+":", err)
		os.Exit(1)
	}

	s := string(data)

	fmt.Println("Lines : ", bytes.Count(data, []byte{'\n'}))
	fmt.Println("words : ", len(strings.Fields(s)))
	fmt.Println("Chars : ", utf8.RuneCount(data))
}
