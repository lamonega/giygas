package tp1.ej1;

public class Ejercicio1 {
	// a. Usando for
    public static void imprimirConFor(int a, int b) {
        int inicio = Math.min(a, b);
        int fin = Math.max(a, b);
        for (int i = inicio; i <= fin; i++) {
            System.out.println(i);
        }
    }

    // b. Usando while
    public static void imprimirConWhile(int a, int b) {
        int inicio = Math.min(a, b);
        int fin = Math.max(a, b);
        int i = inicio;
        while (i <= fin) {
            System.out.println(i);
            i++;
        }
    }

    // c. Usando recursión
    public static void imprimirSinIterativas(int a, int b) {
        System.out.println(a);
        if (a == b) return;
        if (a < b) imprimirSinIterativas(a + 1, b);
        else imprimirSinIterativas(a - 1, b);
    }

    public static void main(String[] args) {
        imprimirConFor(1, 5);
        imprimirConWhile(5, 1);
        imprimirSinIterativas(3, 3);
    }
}
