package tp1.ej2;

import java.util.Scanner;
import java.util.Arrays;

public class Ejercicio2 {
    public static int[] primerosMultiplos(int n) {
        int[] multiplos = new int[n];
        for (int i = 0; i < n; i++) {
            multiplos[i] = n * (i + 1);
        }
        return multiplos;
    }

    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        System.out.print("Ingrese n: ");
        int n = scanner.nextInt();
        scanner.close();
        System.out.println(Arrays.toString(primerosMultiplos(n)));
    }
}
