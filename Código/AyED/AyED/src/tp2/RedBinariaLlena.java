// Ejercicio 4

package tp2;

public class RedBinariaLlena {
    private BinaryTree<Integer> red;

    public RedBinariaLlena(BinaryTree<Integer> unArbolLleno) {
        this.red = unArbolLleno;
    }

    public int retardoReenvio() {
        // Si el árbol está vacío, el retardo es 0
        if (red == null || red.isEmpty()) {
            return 0;
        }
        return calcularRetardo(red);
    }

    private int calcularRetardo(BinaryTree<Integer> nodo) {
        if (nodo == null) {
            return 0;
        }

        // Si es hoja, retorna su propio retardo
        if (nodo.isLeaf()) {
            return nodo.getData();
        }

        // Calcula el retardo de ambos hijos
        int retardoIzquierdo = calcularRetardo(nodo.getLeftChild());
        int retardoDerecho = calcularRetardo(nodo.getRightChild());

        // Suma su propio retardo al mayor camino hacia una hoja
        return nodo.getData() + Math.max(retardoIzquierdo, retardoDerecho);
    }
}