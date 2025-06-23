// Ejercicio 5

package tp2;

public class ProfundidadDelArbolBinario {
    private BinaryTree<Integer> arbol;

    public ProfundidadDelArbolBinario(BinaryTree<Integer> unArbol) {
        this.arbol = unArbol;
    }

    public int sumaElementosProfundidad(int p) {
        if (arbol == null || arbol.isEmpty() || p < 0) {
            return 0;
        }
        return sumarEnProfundidad(arbol, 0, p);
    }

    private int sumarEnProfundidad(BinaryTree<Integer> nodo, int nivelActual, int nivelObjetivo) {
        if (nodo == null) {
            return 0;
        }

        if (nivelActual == nivelObjetivo) {
            return nodo.getData();
        }

        return sumarEnProfundidad(nodo.getLeftChild(), nivelActual + 1, nivelObjetivo) +
               sumarEnProfundidad(nodo.getRightChild(), nivelActual + 1, nivelObjetivo);
    }
}