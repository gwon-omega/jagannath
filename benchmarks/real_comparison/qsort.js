// QuickSort benchmark - Node.js
const SIZE = 1000000;
let arr = new Int32Array(SIZE);

function quicksort(a, low, high) {
  if (low < high) {
    const pivot = a[high];
    let i = low - 1;

    for (let j = low; j < high; j++) {
      if (a[j] <= pivot) {
        i++;
        [a[i], a[j]] = [a[j], a[i]];
      }
    }
    [a[i + 1], a[high]] = [a[high], a[i + 1]];

    const pi = i + 1;
    quicksort(a, low, pi - 1);
    quicksort(a, pi + 1, high);
  }
}

// Simple LCG for reproducible random
function lcg(seed) {
  return (seed * 1103515245 + 12345) >>> 0;
}

const start = performance.now();

for (let run = 0; run < 5; run++) {
  let seed = 42 + run;
  for (let i = 0; i < SIZE; i++) {
    seed = lcg(seed);
    arr[i] = (seed >> 16) % 1000000;
  }
  quicksort(arr, 0, SIZE - 1);
}

const end = performance.now();
console.log(
  `Node.js: QuickSort ${SIZE} elements, Time: ${(end - start).toFixed(
    2
  )} ms (5 runs)`
);
