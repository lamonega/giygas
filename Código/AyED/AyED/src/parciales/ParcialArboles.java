package parciales;
import java.util.*;

import tp3.GeneralTree;

public class ParcialArboles {
    private GeneralTree<Integer> arbol;

    public ParcialArboles(GeneralTree<Integer> arbol) {
        this.arbol = arbol;
    }

    public List<Integer> camino(int num) {
        List<Integer> camino = new ArrayList<>();
        if (arbol != null && !arbol.isEmpty()) {
            buscarCamino(arbol, camino, num);
        }
        return camino;
    }

    private boolean buscarCamino(GeneralTree<Integer> nodo, List<Integer> camino, int num) {
        camino.add(nodo.getData()); // Agrega el nodo actual al camino
        
        if (nodo.isLeaf()) { // Si es hoja, el camino es válido
            return true; 
        } else { 
            List<GeneralTree<Integer>> hijos = nodo.getChildren(); 
            
            if (hijos.size() < num) { // El nodo actual no cumple la condición
                camino.remove(camino.size() - 1); // Backtracking: elimina el nodo del camino
                return false; 
            }
            
            for (GeneralTree<Integer> hijo : hijos) { // Explora cada subárbol hijo
                if (buscarCamino(hijo, camino, num)) { // Busca camino válido en el hijo
                    return true; // Propaga el éxito hacia arriba
                }
            }
            
            // Ningún hijo encontró un camino válido
            camino.remove(camino.size() - 1); // Backtracking: elimina el nodo actual
            return false; 
        }
    }
}

