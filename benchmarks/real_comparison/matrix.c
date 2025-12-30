// Matrix multiplication benchmark - C (Clang)
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define N 512

double A[N][N], B[N][N], C[N][N];

void matrix_mult() {
  for (int i = 0; i < N; i++) {
    for (int j = 0; j < N; j++) {
      double sum = 0.0;
      for (int k = 0; k < N; k++) {
        sum += A[i][k] * B[k][j];
      }
      C[i][j] = sum;
    }
  }
}

int main() {
  // Initialize matrices
  for (int i = 0; i < N; i++) {
    for (int j = 0; j < N; j++) {
      A[i][j] = (double)(i + j) / N;
      B[i][j] = (double)(i - j) / N;
    }
  }

  clock_t start = clock();

  for (int iter = 0; iter < 3; iter++) {
    matrix_mult();
  }

  clock_t end = clock();
  double time_ms = ((double)(end - start) / CLOCKS_PER_SEC) * 1000.0;

  printf("C (Clang): %dx%d matrix mult, Time: %.2f ms (3 runs), C[0][0]=%.4f\n",
         N, N, time_ms, C[0][0]);
  return 0;
}
