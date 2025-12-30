// Fibonacci benchmark - Java
public class Fib {
    static long fib(int n) {
        if (n <= 1) return n;
        return fib(n - 1) + fib(n - 2);
    }

    public static void main(String[] args) {
        // Warmup JIT
        for (int i = 0; i < 3; i++) {
            fib(35);
        }

        long start = System.nanoTime();

        long result = 0;
        for (int i = 0; i < 5; i++) {
            result = fib(40);
        }

        long end = System.nanoTime();
        double timeMs = (end - start) / 1_000_000.0;

        System.out.printf("Java: fib(40) = %d, Time: %.2f ms (5 runs)%n", result, timeMs);
    }
}
