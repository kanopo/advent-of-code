package main

import (
	"fmt"
	"os"
)

func main() {
	dat, err := os.ReadFile("./part_one_test.txt")
	if err != nil {
		fmt.Println(err)
	}

	var data string = string(dat)
	fmt.Println(data)
}
