package main

import (
	// "bufio"
	"fmt"
	// "os"
	"math"
)

func main() {
	low := 1.0
	high := 10000000000.0

	// fmt.Println("Started")
	var n float64
	fmt.Scan(&n)
	// fmt.Println("Started, ", n)

	bin(low, high, float64(n), 0)
}

func bin(low float64, high float64, n float64, iter int) {
	x_guess := (low + high)/2
	// fmt.Println(x_guess)
	n_guess := math.Pow(x_guess, x_guess)

	if math.Abs(n_guess - n) < 0.0000001 || iter > 100 {
		fmt.Println(x_guess)
		// fmt.Println(n_guess)
	} else if n > n_guess {
		bin(x_guess, high, n, iter + 1)
	} else {
		bin(low, x_guess, n, iter + 1)
	}
}
