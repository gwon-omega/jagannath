// Matrix multiplication benchmark - Node.js
const N = 512;

function createMatrix() {
  return Array.from({ length: N }, () => new Float64Array(N));
}

const A = createMatrix();
const B = createMatrix();
const C = createMatrix();

// Initialize
for (let i = 0; i < N; i++) {
  for (let j = 0; j < N; j++) {
    A[i][j] = (i + j) / N;
    B[i][j] = (i - j) / N;
  }
}

function matrixMult() {
  for (let i = 0; i < N; i++) {
    for (let j = 0; j < N; j++) {
      let sum = 0.0;
      for (let k = 0; k < N; k++) {
        sum += A[i][k] * B[k][j];
      }
      C[i][j] = sum;
    }
  }
}

// Warmup
matrixMult();

const start = performance.now();

for (let iter = 0; iter < 3; iter++) {
  matrixMult();
}

const end = performance.now();
console.log(
  `Node.js: ${N}x${N} matrix mult, Time: ${(end - start).toFixed(
    2
  )} ms (3 runs), C[0][0]=${C[0][0].toFixed(4)}`
);
