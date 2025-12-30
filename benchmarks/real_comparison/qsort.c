// QuickSort benchmark - C
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define SIZE 1000000

int arr[SIZE];

void quicksort(int *a, int low, int high) {
  if (low < high) {
    int pivot = a[high];
    int i = low - 1;

    for (int j = low; j < high; j++) {
      if (a[j] <= pivot) {
        i++;
        int temp = a[i];
        a[i] = a[j];
        a[j] = temp;
      }
    }
    int temp = a[i + 1];
    a[i + 1] = a[high];
    a[high] = temp;

    int pi = i + 1;
    quicksort(a, low, pi - 1);
    quicksort(a, pi + 1, high);
  }
}

int main() {
  srand(42);

  clock_t total_start = clock();

  for (int run = 0; run < 5; run++) {
    // Reset array
    for (int i = 0; i < SIZE; i++) {
      arr[i] = rand() % 1000000;
    }
    quicksort(arr, 0, SIZE - 1);
  }

  clock_t total_end = clock();
  double time_ms =
      ((double)(total_end - total_start) / CLOCKS_PER_SEC) * 1000.0;

  printf("C (Clang): QuickSort %d elements, Time: %.2f ms (5 runs)\n", SIZE,
         time_ms);
  return 0;
}
