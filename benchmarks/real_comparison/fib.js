// Fibonacci benchmark - Node.js
function fib(n) {
  if (n <= 1) return n;
  return fib(n - 1) + fib(n - 2);
}

// Warmup V8
for (let i = 0; i < 3; i++) {
  fib(35);
}

const start = performance.now();

let result = 0;
for (let i = 0; i < 5; i++) {
  result = fib(40);
}

const end = performance.now();
console.log(
  `Node.js: fib(40) = ${result}, Time: ${(end - start).toFixed(2)} ms (5 runs)`
);
