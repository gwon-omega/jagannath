// Fibonacci benchmark - C baseline
// Compile: gcc -O3 -march=native -o fibonacci fibonacci.c
// Run: time ./fibonacci 40

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

// Naive recursive fibonacci (for benchmarking)
uint64_t fib_recursive(uint32_t n)
{
  if (n <= 1)
    return n;
  return fib_recursive(n - 1) + fib_recursive(n - 2);
}

// Iterative fibonacci
uint64_t fib_iterative(uint64_t n)
{
  if (n <= 1)
    return n;
  uint64_t a = 0, b = 1;
  for (uint64_t i = 2; i <= n; i++)
  {
    uint64_t temp = a + b;
    a = b;
    b = temp;
  }
  return b;
}

// Matrix exponentiation fibonacci
typedef struct
{
  uint64_t m[2][2];
} Matrix2x2;

Matrix2x2 matrix_multiply(Matrix2x2 a, Matrix2x2 b)
{
  Matrix2x2 result;
  result.m[0][0] = a.m[0][0] * b.m[0][0] + a.m[0][1] * b.m[1][0];
  result.m[0][1] = a.m[0][0] * b.m[0][1] + a.m[0][1] * b.m[1][1];
  result.m[1][0] = a.m[1][0] * b.m[0][0] + a.m[1][1] * b.m[1][0];
  result.m[1][1] = a.m[1][0] * b.m[0][1] + a.m[1][1] * b.m[1][1];
  return result;
}

Matrix2x2 matrix_power(Matrix2x2 base, uint64_t exp)
{
  Matrix2x2 result = {{{1, 0}, {0, 1}}}; // Identity
  while (exp > 0)
  {
    if (exp & 1)
    {
      result = matrix_multiply(result, base);
    }
    base = matrix_multiply(base, base);
    exp >>= 1;
  }
  return result;
}

uint64_t fib_matrix(uint64_t n)
{
  if (n <= 1)
    return n;
  Matrix2x2 base = {{{1, 1}, {1, 0}}};
  Matrix2x2 result = matrix_power(base, n - 1);
  return result.m[0][0];
}

int main(int argc, char *argv[])
{
  uint32_t n = 40;
  if (argc > 1)
  {
    n = atoi(argv[1]);
  }

  printf("Computing fib(%u)...\n", n);

  // Benchmark recursive (warning: slow for n > 40)
  if (n <= 40)
  {
    uint64_t result = fib_recursive(n);
    printf("Recursive: fib(%u) = %lu\n", n, result);
  }

  // Benchmark iterative
  uint64_t result_iter = fib_iterative(n);
  printf("Iterative: fib(%u) = %lu\n", n, result_iter);

  // Benchmark matrix
  uint64_t result_matrix = fib_matrix(n);
  printf("Matrix:    fib(%u) = %lu\n", n, result_matrix);

  return 0;
}
