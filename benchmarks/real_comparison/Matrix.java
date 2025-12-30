// Matrix multiplication benchmark - Java
public class Matrix {
    static final int N = 512;
    static double[][] A = new double[N][N];
    static double[][] B = new double[N][N];
    static double[][] C = new double[N][N];

    static void matrixMult() {
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

    public static void main(String[] args) {
        // Initialize matrices
        for (int i = 0; i < N; i++) {
            for (int j = 0; j < N; j++) {
                A[i][j] = (double)(i + j) / N;
                B[i][j] = (double)(i - j) / N;
            }
        }

        // Warmup
        matrixMult();

        long start = System.nanoTime();

        for (int iter = 0; iter < 3; iter++) {
            matrixMult();
        }

        long end = System.nanoTime();
        double timeMs = (end - start) / 1_000_000.0;

        System.out.printf("Java: %dx%d matrix mult, Time: %.2f ms (3 runs), C[0][0]=%.4f%n",
                          N, N, timeMs, C[0][0]);
    }
}
