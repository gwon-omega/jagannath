// QuickSort benchmark - Go
package main

import (
	"fmt"
	"math/rand"
	"time"
)

const SIZE = 1000000

var arr [SIZE]int

func quicksort(a []int, low, high int) {
	if low < high {
		pivot := a[high]
		i := low - 1

		for j := low; j < high; j++ {
			if a[j] <= pivot {
				i++
				a[i], a[j] = a[j], a[i]
			}
		}
		a[i+1], a[high] = a[high], a[i+1]

		pi := i + 1
		quicksort(a, low, pi-1)
		quicksort(a, pi+1, high)
	}
}

func main() {
	rng := rand.New(rand.NewSource(42))

	start := time.Now()

	for run := 0; run < 5; run++ {
		for i := 0; i < SIZE; i++ {
			arr[i] = rng.Intn(1000000)
		}
		quicksort(arr[:], 0, SIZE-1)
	}

	elapsed := time.Since(start)
	fmt.Printf("Go: QuickSort %d elements, Time: %.2f ms (5 runs)\n", SIZE, float64(elapsed.Milliseconds()))
}
