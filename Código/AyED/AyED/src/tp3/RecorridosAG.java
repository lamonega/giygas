package tp3;
import java.util.*;

public class RecorridosAG {

    public RecorridosAG() {
    }

    public List<Integer> numerosImparesMayoresQuePreOrden (GeneralTree <Integer> a, Integer n) {
        List <Integer> l = new LinkedList<>();
        numerosImparesMayores(a, n, l);
        return l;
    }

    private void numerosImparesMayores(GeneralTree <Integer> a, Integer n, List <Integer> l) {
        int dato = a.getData();
        if(dato %2 != 0 && dato > n) l.add(dato);
        List<GeneralTree<Integer>> children = a.getChildren();
        for(GeneralTree<Integer> child: children) {
            numerosImparesMayores(child, n, l);
        }
    }

    

}
