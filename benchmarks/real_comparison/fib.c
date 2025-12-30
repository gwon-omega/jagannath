// Fibonacci benchmark - C (Clang)
#include <stdio.h>
#include <time.h>

long long fib(int n) {
  if (n <= 1)
    return n;
  return fib(n - 1) + fib(n - 2);
}

int main() {
  clock_t start = clock();

  // Run fibonacci(40) multiple times for more accurate timing
  long long result = 0;
  for (int i = 0; i < 5; i++) {
    result = fib(40);
  }

  clock_t end = clock();
  double time_ms = ((double)(end - start) / CLOCKS_PER_SEC) * 1000.0;

  printf("C (Clang): fib(40) = %lld, Time: %.2f ms (5 runs)\n", result,
         time_ms);
  return 0;
}
