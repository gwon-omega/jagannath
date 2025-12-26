// Matrix multiplication benchmark - C baseline
// Compile: gcc -O3 -march=native -fopenmp -o matrix_mult matrix_mult.c
// Run: time ./matrix_mult 1000

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Naive O(n³) matrix multiplication
void matrix_multiply_naive(double *A, double *B, double *C, int n)
{
  for (int i = 0; i < n; i++)
  {
    for (int j = 0; j < n; j++)
    {
      double sum = 0.0;
      for (int k = 0; k < n; k++)
      {
        sum += A[i * n + k] * B[k * n + j];
      }
      C[i * n + j] = sum;
    }
  }
}

// Cache-optimized (blocked) matrix multiplication
#define BLOCK_SIZE 64

void matrix_multiply_blocked(double *A, double *B, double *C, int n)
{
  memset(C, 0, n * n * sizeof(double));

  for (int ii = 0; ii < n; ii += BLOCK_SIZE)
  {
    for (int jj = 0; jj < n; jj += BLOCK_SIZE)
    {
      for (int kk = 0; kk < n; kk += BLOCK_SIZE)
      {
        // Process block
        int i_end = (ii + BLOCK_SIZE < n) ? ii + BLOCK_SIZE : n;
        int j_end = (jj + BLOCK_SIZE < n) ? jj + BLOCK_SIZE : n;
        int k_end = (kk + BLOCK_SIZE < n) ? kk + BLOCK_SIZE : n;

        for (int i = ii; i < i_end; i++)
        {
          for (int k = kk; k < k_end; k++)
          {
            double a_ik = A[i * n + k];
            for (int j = jj; j < j_end; j++)
            {
              C[i * n + j] += a_ik * B[k * n + j];
            }
          }
        }
      }
    }
  }
}

// Initialize matrix with random values
void init_matrix(double *M, int n)
{
  for (int i = 0; i < n * n; i++)
  {
    M[i] = (double)rand() / RAND_MAX;
  }
}

// Verify result (checksum)
double checksum(double *M, int n)
{
  double sum = 0.0;
  for (int i = 0; i < n * n; i++)
  {
    sum += M[i];
  }
  return sum;
}

int main(int argc, char *argv[])
{
  int n = 1000;
  if (argc > 1)
  {
    n = atoi(argv[1]);
  }

  printf("Matrix multiplication: %d x %d\n", n, n);

  // Allocate matrices
  double *A = (double *)malloc(n * n * sizeof(double));
  double *B = (double *)malloc(n * n * sizeof(double));
  double *C = (double *)malloc(n * n * sizeof(double));

  if (!A || !B || !C)
  {
    fprintf(stderr, "Failed to allocate memory\n");
    return 1;
  }

  // Initialize
  srand(42);
  init_matrix(A, n);
  init_matrix(B, n);

  // Benchmark blocked multiplication
  clock_t start = clock();
  matrix_multiply_blocked(A, B, C, n);
  clock_t end = clock();

  double time_blocked = (double)(end - start) / CLOCKS_PER_SEC;
  printf("Blocked: %.3f seconds (checksum: %.2f)\n", time_blocked, checksum(C, n));

  // Benchmark naive (only for small matrices)
  if (n <= 500)
  {
    memset(C, 0, n * n * sizeof(double));
    start = clock();
    matrix_multiply_naive(A, B, C, n);
    end = clock();

    double time_naive = (double)(end - start) / CLOCKS_PER_SEC;
    printf("Naive:   %.3f seconds (checksum: %.2f)\n", time_naive, checksum(C, n));
  }

  free(A);
  free(B);
  free(C);

  return 0;
}
