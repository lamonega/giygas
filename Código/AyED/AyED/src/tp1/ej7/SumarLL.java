package tp1.ej7;

import java.util.Iterator;
import java.util.LinkedList;

public class SumarLL {
	// Método público que inicia la recursión (interfaz limpia)
    public int sumarLinkedList(LinkedList<Integer> lista) {
        return sumarLinkedList(lista, lista.iterator());
    }
    
    // Método privado recursivo con el Iterator como parámetro
    private int sumarLinkedList(LinkedList<Integer> lista, Iterator<Integer> iter) {
        if (!iter.hasNext()) {
            return 0;
        }
        return iter.next() + sumarLinkedList(lista, iter);
    }
}
