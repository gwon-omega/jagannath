// Matrix multiplication benchmark - Go
package main

import (
	"fmt"
	"time"
)

const N = 512

var A [N][N]float64
var B [N][N]float64
var C [N][N]float64

func matrixMult() {
	for i := 0; i < N; i++ {
		for j := 0; j < N; j++ {
			sum := 0.0
			for k := 0; k < N; k++ {
				sum += A[i][k] * B[k][j]
			}
			C[i][j] = sum
		}
	}
}

func main() {
	// Initialize
	for i := 0; i < N; i++ {
		for j := 0; j < N; j++ {
			A[i][j] = float64(i+j) / N
			B[i][j] = float64(i-j) / N
		}
	}

	start := time.Now()

	for iter := 0; iter < 3; iter++ {
		matrixMult()
	}

	elapsed := time.Since(start)
	fmt.Printf("Go: %dx%d matrix mult, Time: %.2f ms (3 runs), C[0][0]=%.4f\n",
		N, N, float64(elapsed.Milliseconds()), C[0][0])
}
