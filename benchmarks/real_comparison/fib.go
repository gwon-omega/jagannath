// Fibonacci benchmark - Go
package main

import (
	"fmt"
	"time"
)

func fib(n int) int64 {
	if n <= 1 {
		return int64(n)
	}
	return fib(n-1) + fib(n-2)
}

func main() {
	start := time.Now()

	var result int64
	for i := 0; i < 5; i++ {
		result = fib(40)
	}

	elapsed := time.Since(start)
	fmt.Printf("Go: fib(40) = %d, Time: %.2f ms (5 runs)\n", result, float64(elapsed.Milliseconds()))
}
