package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	low := 1
	high := 1000

	bin(low, high)
}

func bin(low int, high int) {
	reader := bufio.NewReader(os.Stdin)
	guess := (low + high)/2 // Will round down
	fmt.Println(guess)

	message, _ := reader.ReadString('\n')
	
	switch message {
	case "higher\n":
		bin(guess+1, high)
	case "lower\n":
		bin(low, guess-1)
	case "correct\n":
		return
	default:
		fmt.Println("ERROR")
		return
	}
}