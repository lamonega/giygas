// Ejercicio 3

package tp2;

import java.util.*;

public class ContadorArbol {
    private BinaryTree<Integer> arbol;

    public ContadorArbol(BinaryTree<Integer> arbol) {
        this.arbol = arbol;
    }

    public List<Integer> numerosParesInOrden() {
        List<Integer> resultado = new ArrayList<>();
        recorrerInOrden(arbol, resultado);
        return resultado;
    }

    private void recorrerInOrden(BinaryTree<Integer> nodo, List<Integer> listaPares) {
        if (nodo == null) {
            return;
        }
        // primero, subárbol izquierdo
        recorrerInOrden(nodo.getLeftChild(), listaPares);


        // luego, el nodo mismo
        if (esPar(nodo.getData())) {
            listaPares.add(nodo.getData());
        }
        
        // finalmente, subárbol derecho
        recorrerInOrden(nodo.getRightChild(), listaPares);
    }

    public List<Integer> numerosParesPostOrden() {
        List<Integer> resultado = new ArrayList<>();
        recorrerPostOrden(arbol, resultado);
        return resultado;
    }

    private void recorrerPostOrden(BinaryTree<Integer> nodo, List<Integer> listaPares) {
        if (nodo == null) {
            return;
        }
        // primero, subárbol izquierdo
        recorrerPostOrden(nodo.getLeftChild(), listaPares);
        // luego, subárbol derecho
        recorrerPostOrden(nodo.getRightChild(), listaPares);
        // por último, el nodo mismo
        if (esPar(nodo.getData())) {
            listaPares.add(nodo.getData());
        }
    }

    private boolean esPar(Integer n) {
        return n % 2 == 0;
    }
}
