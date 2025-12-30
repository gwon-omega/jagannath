// QuickSort benchmark - Java
import java.util.Random;

public class QSort {
    static final int SIZE = 1000000;
    static int[] arr = new int[SIZE];

    static void quicksort(int[] a, int low, int high) {
        if (low < high) {
            int pivot = a[high];
            int i = low - 1;

            for (int j = low; j < high; j++) {
                if (a[j] <= pivot) {
                    i++;
                    int temp = a[i];
                    a[i] = a[j];
                    a[j] = temp;
                }
            }
            int temp = a[i + 1];
            a[i + 1] = a[high];
            a[high] = temp;

            int pi = i + 1;
            quicksort(a, low, pi - 1);
            quicksort(a, pi + 1, high);
        }
    }

    public static void main(String[] args) {
        Random rand = new Random(42);

        // Warmup
        for (int i = 0; i < SIZE; i++) arr[i] = rand.nextInt(1000000);
        quicksort(arr, 0, SIZE - 1);

        rand = new Random(42);
        long start = System.nanoTime();

        for (int run = 0; run < 5; run++) {
            for (int i = 0; i < SIZE; i++) arr[i] = rand.nextInt(1000000);
            quicksort(arr, 0, SIZE - 1);
        }

        long end = System.nanoTime();
        double timeMs = (end - start) / 1_000_000.0;

        System.out.printf("Java: QuickSort %d elements, Time: %.2f ms (5 runs)%n", SIZE, timeMs);
    }
}
