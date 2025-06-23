package tp1.ej7;

import java.util.Iterator;
import java.util.LinkedList;

public class SumarLL {
    public int sumarLinkedList(LinkedList<Integer> lista) {
        return sumarLinkedList(lista, lista.iterator());
    }
    
    private int sumarLinkedList(LinkedList<Integer> lista, Iterator<Integer> iter) {
        if (!iter.hasNext()) {
            return 0;
        }
        return iter.next() + sumarLinkedList(lista, iter);
    }
}
