// Quicksort benchmark - C baseline
// Compile: gcc -O3 -march=native -o quicksort quicksort.c
// Run: time ./quicksort 1000000

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Swap two elements
static inline void swap(int64_t *a, int64_t *b)
{
  int64_t temp = *a;
  *a = *b;
  *b = temp;
}

// Partition for quicksort (Hoare scheme)
int64_t partition_hoare(int64_t *arr, int64_t low, int64_t high)
{
  int64_t pivot = arr[(low + high) / 2];
  int64_t i = low - 1;
  int64_t j = high + 1;

  while (1)
  {
    do
    {
      i++;
    } while (arr[i] < pivot);
    do
    {
      j--;
    } while (arr[j] > pivot);
    if (i >= j)
      return j;
    swap(&arr[i], &arr[j]);
  }
}

// Quicksort (recursive)
void quicksort(int64_t *arr, int64_t low, int64_t high)
{
  if (low < high)
  {
    int64_t p = partition_hoare(arr, low, high);
    quicksort(arr, low, p);
    quicksort(arr, p + 1, high);
  }
}

// Insertion sort for small arrays
void insertion_sort(int64_t *arr, int64_t n)
{
  for (int64_t i = 1; i < n; i++)
  {
    int64_t key = arr[i];
    int64_t j = i - 1;
    while (j >= 0 && arr[j] > key)
    {
      arr[j + 1] = arr[j];
      j--;
    }
    arr[j + 1] = key;
  }
}

// Hybrid quicksort (switches to insertion for small arrays)
#define INSERTION_THRESHOLD 16

void quicksort_hybrid(int64_t *arr, int64_t low, int64_t high)
{
  while (low < high)
  {
    if (high - low < INSERTION_THRESHOLD)
    {
      insertion_sort(arr + low, high - low + 1);
      return;
    }

    int64_t p = partition_hoare(arr, low, high);

    // Tail call optimization: recurse on smaller partition
    if (p - low < high - p)
    {
      quicksort_hybrid(arr, low, p);
      low = p + 1;
    }
    else
    {
      quicksort_hybrid(arr, p + 1, high);
      high = p;
    }
  }
}

// Verify sorted
int is_sorted(int64_t *arr, int64_t n)
{
  for (int64_t i = 1; i < n; i++)
  {
    if (arr[i] < arr[i - 1])
      return 0;
  }
  return 1;
}

// Fisher-Yates shuffle
void shuffle(int64_t *arr, int64_t n)
{
  for (int64_t i = n - 1; i > 0; i--)
  {
    int64_t j = rand() % (i + 1);
    swap(&arr[i], &arr[j]);
  }
}

int main(int argc, char *argv[])
{
  int64_t n = 1000000;
  if (argc > 1)
  {
    n = atoll(argv[1]);
  }

  printf("Quicksort: %ld elements\n", n);

  // Allocate and initialize
  int64_t *arr = (int64_t *)malloc(n * sizeof(int64_t));
  int64_t *arr_copy = (int64_t *)malloc(n * sizeof(int64_t));

  if (!arr || !arr_copy)
  {
    fprintf(stderr, "Failed to allocate memory\n");
    return 1;
  }

  // Initialize with sequential values, then shuffle
  for (int64_t i = 0; i < n; i++)
  {
    arr[i] = i;
  }
  srand(42);
  shuffle(arr, n);
  memcpy(arr_copy, arr, n * sizeof(int64_t));

  // Benchmark basic quicksort
  clock_t start = clock();
  quicksort(arr, 0, n - 1);
  clock_t end = clock();

  double time_basic = (double)(end - start) / CLOCKS_PER_SEC;
  printf("Basic:  %.3f seconds (sorted: %s)\n", time_basic, is_sorted(arr, n) ? "yes" : "no");

  // Benchmark hybrid quicksort
  memcpy(arr, arr_copy, n * sizeof(int64_t));
  start = clock();
  quicksort_hybrid(arr, 0, n - 1);
  end = clock();

  double time_hybrid = (double)(end - start) / CLOCKS_PER_SEC;
  printf("Hybrid: %.3f seconds (sorted: %s)\n", time_hybrid, is_sorted(arr, n) ? "yes" : "no");

  free(arr);
  free(arr_copy);

  return 0;
}
